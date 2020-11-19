'''
File: afc.py
Project: python
Created Date: 09/05/2020
Author: Shun Suzuki
-----
Last Modified: 19/11/2020
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2020 Hapis Lab. All rights reserved.

'''

from enum import IntEnum

import ctypes
from ctypes import c_void_p, byref, c_uint, POINTER, c_float, c_ulong
import numpy as np

from . import nativemethods
from .nativemethods import SphereWaveSource, T4010A1, Vector3


class Axis(IntEnum):
    X = 1
    Y = 2
    Z = 3


class UniformSystem:
    def __init__(self, temperature):
        self.handle = c_void_p()
        self.temperature = temperature
        self._source_type = -1

    def __del__(self):
        if self.handle:
            nativemethods.AFC_DLL.AFC_FreeUniformSystem(self.handle, self._source_type)

    def set_source_type(self, v):
        self._source_type = v

    def __check_and_init(self, source):
        if self._source_type == -1:
            self._source_type = source.get_type()
            nativemethods.AFC_DLL.AFC_CreateUniformSystem(byref(self.handle), self.temperature, self._source_type)
        elif self._source_type != source.get_type():
            raise TypeError("Source type error! Mixing sound source types is not allowed.")

    def sound_speed(self):
        if self.handle:
            return nativemethods.AFC_DLL.AFC_UniformSystemSoundSpeed(self.handle, self._source_type)
        else:
            raise TypeError("Please add sound source before.")

    def info(self):
        if self.handle:
            info = nativemethods.AFC_DLL.AFC_UniformSystemInfo(self.handle, self._source_type)
            print(info.decode('utf-8'))

    def info_of_source(self, idx: int):
        if self.handle:
            info = nativemethods.AFC_DLL.AFC_UniformSystemSourceInfo(self.handle, idx, self._source_type)
            print(info.decode('utf-8'))

    def add_wave_source(self, source):
        self.__check_and_init(source)
        if isinstance(source, SphereWaveSource):
            nativemethods.AFC_DLL.AFC_AddSphereWaveSource(self.handle, source)
        elif isinstance(source, T4010A1):
            nativemethods.AFC_DLL.AFC_AddT4010A1(self.handle, source)
        else:
            raise TypeError('Unknown source type')

    def get_wave_sources(self):
        ptr = c_void_p()
        size = nativemethods.AFC_DLL.AFC_GetWaveSources(self.handle, byref(ptr), self._source_type)
        if self._source_type == 0:
            array = ctypes.cast(ptr, POINTER(SphereWaveSource * size)).contents
            return array
        elif self._source_type == 1:
            array = ctypes.cast(ptr, POINTER(T4010A1 * size)).contents
            return array
        else:
            raise TypeError('Unknown source type')


class Calculator:
    def __init__(self):
        self.handle = c_void_p()
        nativemethods.AFC_DLL.AFC_CreateCalculator(byref(self.handle), self.get_type())

    def __del__(self):
        if self.handle:
            nativemethods.AFC_DLL.AFC_FreeCalculator(self.handle, self.get_type())

    def calculate(self, system, observe_area, field):
        ptr = c_void_p()
        ft = field.get_type()
        if ft == 2:
            size = nativemethods.AFC_DLL.AFC_Calculate(self.handle, system.handle, observe_area.handle, field.handle,
                                                       byref(ptr), self.get_type(), system._source_type, observe_area.get_type(), ft)
            ptr = ctypes.cast(ptr, ctypes.POINTER(c_float))
            carray = np.ctypeslib.as_array(ptr, shape=(size, 2))
            return np.apply_along_axis(lambda x: complex(x[0], x[1]), 1, carray)

        else:
            size = nativemethods.AFC_DLL.AFC_Calculate(self.handle, system.handle, observe_area.handle, field.handle,
                                                       byref(ptr), self.get_type(), system._source_type, observe_area.get_type(), ft)
            ptr = ctypes.cast(ptr, ctypes.POINTER(c_float))
            return np.ctypeslib.as_array(ptr, shape=(size,))

    @ classmethod
    def get_type():
        raise NotImplementedError()


class CpuCalculator(Calculator):
    def __init__(self):
        super().__init__()

    def __del__(self):
        super().__del__()

    @ staticmethod
    def get_type():
        return 0


class AccurateCalculator(Calculator):
    def __init__(self):
        super().__init__()

    def __del__(self):
        super().__del__()

    @ staticmethod
    def get_type():
        return 1


class GpuCalculator(Calculator):
    def __init__(self):
        super().__init__()

    def __del__(self):
        super().__del__()

    @ staticmethod
    def get_type():
        return 2


class FieldBuffer:
    def __init__(self):
        self.handle = c_void_p()

    def __del__(self):
        pass

    @ classmethod
    def get_type():
        raise NotImplementedError()


class PressureFieldBuffer(FieldBuffer):
    def __init__(self):
        super().__init__()
        nativemethods.AFC_DLL.AFC_CreatePressureField(byref(self.handle))

    def __del__(self):
        nativemethods.AFC_DLL.AFC_FreePressureField(self.handle)

    @ staticmethod
    def get_type():
        return 0


class PowerFieldBuffer(FieldBuffer):
    def __init__(self):
        super().__init__()
        nativemethods.AFC_DLL.AFC_CreatePowerField(byref(self.handle))

    def __del__(self):
        nativemethods.AFC_DLL.AFC_FreePowerField(self.handle)

    @ staticmethod
    def get_type():
        return 1


class ComplexPressureFieldBuffer(FieldBuffer):
    def __init__(self):
        super().__init__()
        nativemethods.AFC_DLL.AFC_CreateComplexPressureField(byref(self.handle))

    def __del__(self):
        nativemethods.AFC_DLL.AFC_FreeComplexPressureField(self.handle)

    @ staticmethod
    def get_type():
        return 2


class ObserveArea():
    def __init__(self):
        self.handle = c_void_p()

    def __del__(self):
        pass

    @ classmethod
    def get_type():
        raise NotImplementedError()


class ScatterArea(ObserveArea):
    def __init__(self):
        super().__init__()
        nativemethods.AFC_DLL.AFC_CreateScatterArea(byref(self.handle))

    def __del__(self):
        nativemethods.AFC_DLL.AFC_FreeScatterArea(self.handle)

    def add_observe_point(self, p: Vector3):
        nativemethods.AFC_DLL.AFC_ScatterAddObservePoint(self.handle, p)

    @ staticmethod
    def get_type():
        return 3


class GridArea():
    def __init__(self):
        self.handle = c_void_p()

    def __del__(self):
        if self.handle:
            nativemethods.AFC_DLL.AFC_FreeGridArea(self.handle, self.get_dimension())

    def get_size(self):
        f = c_uint()
        s = c_uint()
        t = c_uint()
        nativemethods.AFC_DLL.AFC_GetGridSize(self.handle, self.get_dimension(), byref(f), byref(s), byref(t))
        return (f.value, s.value, t.value)

    @ classmethod
    def get_type():
        raise NotImplementedError()

    @ classmethod
    def get_dimension():
        raise NotImplementedError()


class GridArea1D(GridArea):
    def __init__(self):
        super().__init__()

    def __del__(self):
        super().__del__()

    @ staticmethod
    def get_type():
        return 0

    @ staticmethod
    def get_dimension():
        return 1


class GridArea2D(GridArea):
    def __init__(self):
        super().__init__()

    def __del__(self):
        super().__del__()

    @ staticmethod
    def get_type():
        return 1

    @ staticmethod
    def get_dimension():
        return 2


class GridArea3D(GridArea):
    def __init__(self):
        super().__init__()

    def __del__(self):
        super().__del__()

    @ staticmethod
    def get_type():
        return 2

    @ staticmethod
    def get_dimension():
        return 3


class GridAreaBuilder:
    def __init__(self):
        self.__dim_cnt = 0
        self.__ranges = []
        self.__resolution = None
        self.__axes = []
        pass

    def __del__(self):
        pass

    def __add_axes(self, axis):
        if axis in self.__axes:
            raise RuntimeError(f'The axis ({axis}) is already specified.')
        self.__axes.append(axis)

    def x_at(self, pos: float):
        self.__ranges.append((pos, pos))
        self.__add_axes(Axis.X)
        return self

    def y_at(self, pos: float):
        self.__ranges.append((pos, pos))
        self.__add_axes(Axis.Y)
        return self

    def z_at(self, pos: float):
        self.__ranges.append((pos, pos))
        self.__add_axes(Axis.Z)
        return self

    def x_range(self, obs_range: (float, float)):
        self.__ranges.append(obs_range)
        self.__add_axes(Axis.X)
        self.__dim_cnt += 1
        return self

    def y_range(self, obs_range: (float, float)):
        self.__ranges.append(obs_range)
        self.__add_axes(Axis.Y)
        self.__dim_cnt += 1
        return self

    def z_range(self, obs_range: (float, float)):
        self.__ranges.append(obs_range)
        self.__add_axes(Axis.Z)
        self.__dim_cnt += 1
        return self

    def resolution(self, r: float):
        if self.__resolution is None:
            self.__resolution = r
        else:
            raise RuntimeError('Resolution is already specified.')
        return self

    def generate(self):
        if len(self.__ranges) != 3:
            raise RuntimeError('Some ranges is not specified.')
        if self.__resolution is None:
            raise RuntimeError('Resolution is not specified.')

        fr = self.__ranges[0]
        sr = self.__ranges[1]
        tr = self.__ranges[2]

        if self.__dim_cnt == 0 or self.__dim_cnt == 1:
            area = GridArea1D()
            nativemethods.AFC_DLL.AFC_CreateGridArea1D(
                byref(
                    area.handle),
                fr[0],
                fr[1],
                sr[0],
                tr[0],
                self.__resolution,
                self.__axes[0],
                self.__axes[1],
                self.__axes[2])
            return area

        elif self.__dim_cnt == 2:
            area = GridArea2D()
            nativemethods.AFC_DLL.AFC_CreateGridArea2D(
                byref(
                    area.handle),
                fr[0],
                fr[1],
                sr[0],
                sr[1],
                tr[0],
                self.__resolution,
                self.__axes[0],
                self.__axes[1],
                self.__axes[2])
            return area
        elif self.__dim_cnt == 3:
            area = GridArea3D()
            nativemethods.AFC_DLL.AFC_CreateGridArea3D(
                byref(
                    area.handle),
                fr[0],
                fr[1],
                sr[0],
                sr[1],
                tr[0],
                tr[1],
                self.__resolution,
                self.__axes[0],
                self.__axes[1],
                self.__axes[2])
            return area


class Optimizer():
    @ staticmethod
    def focus(system: UniformSystem, p):
        if isinstance(p, np.ndarray):
            p = Vector3(p)
        nativemethods.AFC_DLL.AFO_FocalPoint(system.handle, p, system._source_type)

    @ staticmethod
    def bessel(system: UniformSystem, p, direction, theta_z: float):
        if isinstance(p, np.ndarray):
            p = Vector3(p)
        if isinstance(direction, np.ndarray):
            direction = Vector3(direction)
        nativemethods.AFC_DLL.AFO_BesselBeam(system.handle, p, direction, c_float(theta_z), system._source_type)

    @ staticmethod
    def ifft(system: UniformSystem, path, bottom_left, top_left, bottom_right, spacing, z):
        nativemethods.AFC_DLL.AFO_IFFT(system.handle, path.encode('utf-8'), bottom_left, top_left, bottom_right,
                                       spacing, z, system._source_type)

    @ staticmethod
    def gs_pat(system: UniformSystem, foci, amps):
        size = len(foci)
        amps = np.array(amps).astype(np.float32)
        amps = np.ctypeslib.as_ctypes(amps)
        foci_array = np.zeros([size * 3]).astype(np.float32)
        for i, focus in enumerate(foci):
            foci_array[3 * i] = focus[0]
            foci_array[3 * i + 1] = focus[1]
            foci_array[3 * i + 2] = focus[2]
        foci_array = np.ctypeslib.as_ctypes(foci_array)
        nativemethods.AFC_DLL.AFO_GSPAT(system.handle, foci_array, amps, c_ulong(size), system._source_type)

    @ staticmethod
    def gs(system: UniformSystem, foci, amps):
        size = len(foci)
        amps = np.array(amps).astype(np.float32)
        amps = np.ctypeslib.as_ctypes(amps)
        foci_array = np.zeros([size * 3]).astype(np.float32)
        for i, focus in enumerate(foci):
            foci_array[3 * i] = focus[0]
            foci_array[3 * i + 1] = focus[1]
            foci_array[3 * i + 2] = focus[2]
        foci_array = np.ctypeslib.as_ctypes(foci_array)
        nativemethods.AFC_DLL.AFO_GS(system.handle, foci_array, amps, c_ulong(size), system._source_type)

    @ staticmethod
    def naive(system: UniformSystem, foci, amps):
        size = len(foci)
        amps = np.array(amps).astype(np.float32)
        amps = np.ctypeslib.as_ctypes(amps)
        foci_array = np.zeros([size * 3]).astype(np.float32)
        for i, focus in enumerate(foci):
            foci_array[3 * i] = focus[0]
            foci_array[3 * i + 1] = focus[1]
            foci_array[3 * i + 2] = focus[2]
        foci_array = np.ctypeslib.as_ctypes(foci_array)
        nativemethods.AFC_DLL.AFO_Naive(system.handle, foci_array, amps, c_ulong(size), system._source_type)

    @ staticmethod
    def horn(system: UniformSystem, foci, amps):
        size = len(foci)
        amps = np.array(amps).astype(np.float32)
        amps = np.ctypeslib.as_ctypes(amps)
        foci_array = np.zeros([size * 3]).astype(np.float32)
        for i, focus in enumerate(foci):
            foci_array[3 * i] = focus[0]
            foci_array[3 * i + 1] = focus[1]
            foci_array[3 * i + 2] = focus[2]
        foci_array = np.ctypeslib.as_ctypes(foci_array)
        nativemethods.AFC_DLL.AFO_Horn(system.handle, foci_array, amps, c_ulong(size), system._source_type)

    @ staticmethod
    def long2014(system: UniformSystem, foci, amps):
        size = len(foci)
        amps = np.array(amps).astype(np.float32)
        amps = np.ctypeslib.as_ctypes(amps)
        foci_array = np.zeros([size * 3]).astype(np.float32)
        for i, focus in enumerate(foci):
            foci_array[3 * i] = focus[0]
            foci_array[3 * i + 1] = focus[1]
            foci_array[3 * i + 2] = focus[2]
        foci_array = np.ctypeslib.as_ctypes(foci_array)
        nativemethods.AFC_DLL.AFO_Long(system.handle, foci_array, amps, c_ulong(size), system._source_type)

    @ staticmethod
    def apo(system: UniformSystem, foci, amps, lambda_reg: float):
        size = len(foci)
        amps = np.array(amps).astype(np.float32)
        amps = np.ctypeslib.as_ctypes(amps)
        foci_array = np.zeros([size * 3]).astype(np.float32)
        for i, focus in enumerate(foci):
            foci_array[3 * i] = focus[0]
            foci_array[3 * i + 1] = focus[1]
            foci_array[3 * i + 2] = focus[2]
        foci_array = np.ctypeslib.as_ctypes(foci_array)
        nativemethods.AFC_DLL.AFO_APO(system.handle, foci_array, amps, c_ulong(size), lambda_reg, system._source_type)

    @ staticmethod
    def lm(system: UniformSystem, foci, amps):
        size = len(foci)
        amps = np.array(amps).astype(np.float32)
        amps = np.ctypeslib.as_ctypes(amps)
        foci_array = np.zeros([size * 3]).astype(np.float32)
        for i, focus in enumerate(foci):
            foci_array[3 * i] = focus[0]
            foci_array[3 * i + 1] = focus[1]
            foci_array[3 * i + 2] = focus[2]
        foci_array = np.ctypeslib.as_ctypes(foci_array)
        nativemethods.AFC_DLL.AFO_LM(system.handle, foci_array, amps, c_ulong(size), system._source_type)

    @ staticmethod
    def gauss_newton(system: UniformSystem, foci, amps):
        size = len(foci)
        amps = np.array(amps).astype(np.float32)
        amps = np.ctypeslib.as_ctypes(amps)
        foci_array = np.zeros([size * 3]).astype(np.float32)
        for i, focus in enumerate(foci):
            foci_array[3 * i] = focus[0]
            foci_array[3 * i + 1] = focus[1]
            foci_array[3 * i + 2] = focus[2]
        foci_array = np.ctypeslib.as_ctypes(foci_array)
        nativemethods.AFC_DLL.AFO_GaussNewton(system.handle, foci_array, amps, c_ulong(size), system._source_type)

    @ staticmethod
    def gradient_descent(system: UniformSystem, foci, amps):
        size = len(foci)
        amps = np.array(amps).astype(np.float32)
        amps = np.ctypeslib.as_ctypes(amps)
        foci_array = np.zeros([size * 3]).astype(np.float32)
        for i, focus in enumerate(foci):
            foci_array[3 * i] = focus[0]
            foci_array[3 * i + 1] = focus[1]
            foci_array[3 * i + 2] = focus[2]
        foci_array = np.ctypeslib.as_ctypes(foci_array)
        nativemethods.AFC_DLL.AFO_GradientDescent(system.handle, foci_array, amps, c_ulong(size), system._source_type)
