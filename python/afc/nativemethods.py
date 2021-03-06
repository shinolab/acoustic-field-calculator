'''
File: nativemethods.py
Project: python
Created Date: 09/05/2020
Author: Shun Suzuki
-----
Last Modified: 17/06/2021
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2020 Hapis Lab. All rights reserved.

'''

import ctypes
from ctypes import c_void_p, c_int, POINTER, c_float, c_ulong, c_uint, Structure, c_char_p


class Vector3(Structure):
    _fields_ = [("x", c_float), ("y", c_float), ("z", c_float)]

    def __init__(self, position):
        super().__init__()
        self.x = position[0]
        self.y = position[1]
        self.z = position[2]


class SphereWaveSource(Structure):
    _fields_ = [("x", c_float), ("y", c_float), ("z", c_float), ("amp", c_float), ("phase", c_float), ("freq", c_float)]

    def __init__(self, position, amp, phase, freq):
        super().__init__()
        self.pos = position
        self.amp = amp
        self.phase = phase
        self.freq = freq

    @property
    def pos(self):
        pass

    @pos.setter
    def pos(self, position):
        self.x = position[0]
        self.y = position[1]
        self.z = position[2]

    @pos.getter
    def pos(self):
        return (self.x, self.y, self.z)

    @staticmethod
    def get_type():
        return 0


class T4010A1(Structure):
    _fields_ = [("x", c_float), ("y", c_float), ("z", c_float), ("nx", c_float), ("ny", c_float),
                ("nz", c_float), ("amp", c_float), ("phase", c_float), ("freq", c_float)]

    def __init__(self, position, direction, amp, phase, freq):
        super().__init__()
        self.pos = position
        self.direction = direction
        self.amp = amp
        self.phase = phase
        self.freq = freq

    @property
    def pos(self):
        pass

    @pos.setter
    def pos(self, position):
        self.x = position[0]
        self.y = position[1]
        self.z = position[2]

    @pos.getter
    def pos(self):
        return (self.x, self.y, self.z)

    @property
    def direction(self):
        pass

    @direction.setter
    def direction(self, direction):
        self.nx = direction[0]
        self.ny = direction[1]
        self.nz = direction[2]

    @direction.getter
    def dir(self):
        return (self.nx, self.ny, self.nz)

    @staticmethod
    def get_type():
        return 1


def init_dll(dll_location):
    global AFC_DLL  # pylint: disable=global-variable-undefined
    AFC_DLL = ctypes.CDLL(dll_location)

    __init_calculator()
    __init_field_buffer()
    __init_observe_area()
    __init_optimizer()
    __init_system()


def __init_calculator():
    AFC_DLL.AFC_CreateCalculator.argtypes = [POINTER(c_void_p), c_int]
    AFC_DLL.AFC_CreateCalculator.restype = None

    AFC_DLL.AFC_FreeCalculator.argtypes = [c_void_p, c_int]
    AFC_DLL.AFC_FreeCalculator.restype = None

    AFC_DLL.AFC_Calculate.argtypes = [c_void_p, c_void_p, c_void_p, c_void_p, POINTER(c_void_p), c_int, c_int, c_int, c_int]
    AFC_DLL.AFC_Calculate.restype = c_ulong


def __init_field_buffer():
    AFC_DLL.AFC_CreatePressureField.argtypes = [POINTER(c_void_p)]
    AFC_DLL.AFC_CreatePressureField.restype = None

    AFC_DLL.AFC_FreePressureField.argtypes = [c_void_p]
    AFC_DLL.AFC_FreePressureField.restype = None

    AFC_DLL.AFC_CreatePowerField.argtypes = [POINTER(c_void_p)]
    AFC_DLL.AFC_CreatePowerField.restype = None

    AFC_DLL.AFC_FreePowerField.argtypes = [c_void_p]
    AFC_DLL.AFC_FreePowerField.restype = None

    AFC_DLL.AFC_CreateComplexPressureField.argtypes = [POINTER(c_void_p)]
    AFC_DLL.AFC_CreateComplexPressureField.restype = None

    AFC_DLL.AFC_FreeComplexPressureField.argtypes = [c_void_p]
    AFC_DLL.AFC_FreeComplexPressureField.restype = None


def __init_observe_area():
    AFC_DLL.AFC_CreateScatterArea.argtypes = [POINTER(c_void_p)]
    AFC_DLL.AFC_CreateScatterArea.restype = None

    AFC_DLL.AFC_ScatterAddObservePoint.argtypes = [c_void_p, Vector3]
    AFC_DLL.AFC_ScatterAddObservePoint.restype = None

    AFC_DLL.AFC_FreeScatterArea.argtypes = [c_void_p]
    AFC_DLL.AFC_FreeScatterArea.restype = None

    AFC_DLL.AFC_CreateGridArea1D.argtypes = [POINTER(c_void_p), c_float, c_float, c_float, c_float, c_float, c_int, c_int, c_int]
    AFC_DLL.AFC_CreateGridArea1D.restype = None

    AFC_DLL.AFC_CreateGridArea2D.argtypes = [POINTER(c_void_p), c_float, c_float, c_float, c_float, c_float, c_float, c_int, c_int, c_int]
    AFC_DLL.AFC_CreateGridArea2D.restype = None

    AFC_DLL.AFC_CreateGridArea3D.argtypes = [POINTER(c_void_p), c_float, c_float, c_float, c_float, c_float, c_float, c_float, c_int, c_int, c_int]
    AFC_DLL.AFC_CreateGridArea3D.restype = None

    AFC_DLL.AFC_FreeGridArea.argtypes = [c_void_p, c_int]
    AFC_DLL.AFC_FreeGridArea.restype = None

    AFC_DLL.AFC_GetGridSize.argtypes = [c_void_p, c_int, POINTER(c_uint), POINTER(c_uint), POINTER(c_uint)]
    AFC_DLL.AFC_GetGridSize.restype = None


def __init_optimizer():
    AFC_DLL.AFO_FocalPoint.argtypes = [c_void_p, Vector3, c_int]
    AFC_DLL.AFO_FocalPoint.restype = None

    AFC_DLL.AFO_BesselBeam.argtypes = [c_void_p, Vector3, Vector3, c_float, c_int]
    AFC_DLL.AFO_BesselBeam.restype = None

    AFC_DLL.AFO_IFFT.argtypes = [c_void_p, c_char_p, Vector3, Vector3, Vector3, c_float, c_float, c_int]
    AFC_DLL.AFO_IFFT.restype = None

    AFC_DLL.AFO_GSPAT.argtypes = [c_void_p, c_void_p, POINTER(c_float), c_ulong, c_int]
    AFC_DLL.AFO_GSPAT.restype = None

    AFC_DLL.AFO_GS.argtypes = [c_void_p, c_void_p, POINTER(c_float), c_ulong, c_int]
    AFC_DLL.AFO_GS.restype = None

    AFC_DLL.AFO_Naive.argtypes = [c_void_p, c_void_p, POINTER(c_float), c_ulong, c_int]
    AFC_DLL.AFO_Naive.restype = None

    AFC_DLL.AFO_Horn.argtypes = [c_void_p, c_void_p, POINTER(c_float), c_ulong, c_int]
    AFC_DLL.AFO_Horn.restype = None

    AFC_DLL.AFO_Long.argtypes = [c_void_p, c_void_p, POINTER(c_float), c_ulong, c_int]
    AFC_DLL.AFO_Long.restype = None

    AFC_DLL.AFO_APO.argtypes = [c_void_p, c_void_p, POINTER(c_float), c_ulong, c_float, c_int]
    AFC_DLL.AFO_APO.restype = None

    AFC_DLL.AFO_GaussNewton.argtypes = [c_void_p, c_void_p, POINTER(c_float), c_ulong, c_int]
    AFC_DLL.AFO_GaussNewton.restype = None

    AFC_DLL.AFO_LM.argtypes = [c_void_p, c_void_p, POINTER(c_float), c_ulong, c_int]
    AFC_DLL.AFO_LM.restype = None

    AFC_DLL.AFO_GradientDescent.argtypes = [c_void_p, c_void_p, POINTER(c_float), c_ulong, c_int]
    AFC_DLL.AFO_GradientDescent.restype = None

    AFC_DLL.AFO_Greedy.argtypes = [c_void_p, c_void_p, POINTER(c_float), c_ulong, c_ulong, c_int]
    AFC_DLL.AFO_Greedy.restype = None


def __init_system():
    AFC_DLL.AFC_CreateUniformSystem.argtypes = [c_void_p, c_float, c_int]
    AFC_DLL.AFC_CreateUniformSystem.restype = None

    AFC_DLL.AFC_FreeUniformSystem.argtypes = [c_void_p, c_int]
    AFC_DLL.AFC_FreeUniformSystem.restype = None

    AFC_DLL.AFC_AddSphereWaveSource.argtypes = [c_void_p, SphereWaveSource]
    AFC_DLL.AFC_AddSphereWaveSource.restype = None

    AFC_DLL.AFC_AddT4010A1.argtypes = [c_void_p, T4010A1]
    AFC_DLL.AFC_AddT4010A1.restype = None

    AFC_DLL.AFC_GetWaveSources.argtypes = [c_void_p, POINTER(c_void_p), c_int]
    AFC_DLL.AFC_GetWaveSources.restype = c_ulong

    AFC_DLL.AFC_UniformSystemSoundSpeed.argtypes = [c_void_p, c_int]
    AFC_DLL.AFC_UniformSystemSoundSpeed.restype = c_float

    AFC_DLL.AFC_UniformSystemInfo.argtypes = [c_void_p, c_int]
    AFC_DLL.AFC_UniformSystemInfo.restype = c_char_p

    AFC_DLL.AFC_UniformSystemSourceInfo.argtypes = [c_void_p, c_ulong, c_int]
    AFC_DLL.AFC_UniformSystemSourceInfo.restype = c_char_p
