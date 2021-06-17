'''
File: __init__.py
Project: afc
Created Date: 09/05/2020
Author: Shun Suzuki
-----
Last Modified: 17/06/2021
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2020 Hapis Lab. All rights reserved.

'''

import os.path
import platform
import requests
import shutil

from . import plot_helper
from .afc import UniformSystem
from .afc import CpuCalculator, AccurateCalculator, GpuCalculator
from .afc import PressureFieldBuffer, PowerFieldBuffer, ComplexPressureFieldBuffer
from .afc import ScatterArea, GridAreaBuilder
from .afc import Optimizer
from .nativemethods import init_dll, SphereWaveSource, T4010A1, Vector3

__all__ = [
    'UniformSystem',
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

_Version = '0.6.1'

PLATFORM = platform.system()
PREFIX = ''
EXT = ''
if PLATFORM == 'Windows':
    EXT = '.dll'
elif PLATFORM == 'Linux':
    PREFIX = 'lib'
    EXT = '.so'

LIB_DIR_PATH = os.path.join(os.path.dirname(__file__), 'bin')
LIB_PATH = os.path.join(LIB_DIR_PATH, PREFIX + 'afccapi-' + _Version + EXT)


def download_bin():
    if not os.path.isdir(LIB_DIR_PATH):
        os.mkdir(LIB_DIR_PATH)
    response = requests.get(f'https://github.com/shinolab/acoustic-field-calculator/releases/download/v{_Version}/{PREFIX}afccapi{EXT}', stream=True)
    with open(LIB_PATH, 'wb') as fp:
        shutil.copyfileobj(response.raw, fp)


if not os.path.isfile(LIB_PATH):
    print('Dll does not exist. Downloading latest version...')
    download_bin()


init_dll(LIB_PATH)
