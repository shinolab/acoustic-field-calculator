'''
File: __init__.py
Project: afc
Created Date: 09/05/2020
Author: Shun Suzuki
-----
Last Modified: 16/11/2020
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2020 Hapis Lab. All rights reserved.

'''

import os.path

from afc import plot_helper
from .afc import CpuCalculator, AccurateCalculator, GpuCalculator
from .afc import PressureFieldBuffer, PowerFieldBuffer, ComplexPressureFieldBuffer
from .afc import ScatterArea, GridAreaBuilder
from .afc import Optimizer
from .nativemethods import init_dll, SphereWaveSource, T4010A1, Vector3

__all__ = [
    'CpuCalculator',
    'AccurateCalculator',
    'GpuCalculator',
    'PressureFieldBuffer',
    'PowerFieldBuffer',
    'ComplexPressureFieldBuffer',
    'ScatterArea',
    'GridAreaBuilder',
    'SphereWaveSource',
    'T4010A1',
    'Vector3',
    'Optimizer',
    'plot_helper']

_Version = '0.5.5'
LIB_DIR_PATH = os.path.join(os.path.dirname(__file__), 'bin')
LIB_PATH = os.path.join(LIB_DIR_PATH, 'afccapi.dll')

init_dll(LIB_PATH)
