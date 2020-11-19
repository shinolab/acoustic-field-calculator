'''
File: foci.py
Project: python
Created Date: 22/09/2020
Author: Shun Suzuki
-----
Last Modified: 19/11/2020
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2020 Hapis Lab. All rights reserved.

'''

import os  # NOQA
import math  # NOQA
import matplotlib.pyplot as plt
import numpy as np
import mpl_toolkits.axes_grid1

from afc import UniformSystem
from afc import SphereWaveSource, T4010A1  # NOQA
from afc import CpuCalculator, GpuCalculator, GridAreaBuilder, PressureFieldBuffer, PowerFieldBuffer  # NOQA
from afc import Optimizer
from afc import Vector3  # NOQA
from afc import plot_helper


def plot(bounds, results, name, ext='png', show=True):
    array = result.reshape(bounds[0], bounds[1])
    DPI = 72
    fig = plt.figure(figsize=(6, 6), dpi=DPI)
    axes = fig.add_subplot(111, aspect='equal')
    heat_map = plot_helper.plot_acoustic_field_2d(axes, array, X_RANGE, Y_RANGE, RESOLUTION, ticks_step=10.0)
    divider = mpl_toolkits.axes_grid1.make_axes_locatable(axes)
    cax = divider.append_axes('right', '5%', pad='3%')
    fig.colorbar(heat_map, cax=cax)
    plt.savefig(f'{name}.{ext}')
    if show:
        plt.title(name)
        plt.show()


if __name__ == '__main__':
    NUM_TRANS_X = 18
    NUM_TRANS_Y = 14
    TRANS_SIZE = 10.18
    FREQUENCY = 40e3
    TEMPERATURE = 300
    Z_DIR = np.array([0, 0, 1])

    array_center = np.array([TRANS_SIZE * (NUM_TRANS_X - 1) / 2.0, TRANS_SIZE * (NUM_TRANS_Y - 1) / 2.0, 0])

    calculator = CpuCalculator()

    system = UniformSystem(TEMPERATURE)
    for y in range(NUM_TRANS_Y):
        for x in range(NUM_TRANS_X):
            pos = np.array([TRANS_SIZE * x, TRANS_SIZE * y, 0.]) - array_center
            source = T4010A1(pos, Z_DIR, 1.0, 0.0, FREQUENCY)
            system.add_wave_source(source)

    z = 150.0

    R = 200.0
    X_RANGE = (-R / 2, R / 2)
    Y_RANGE = (-R / 2, R / 2)
    RESOLUTION = 1.0

    observe_area = GridAreaBuilder()\
        .x_range(X_RANGE)\
        .y_range(Y_RANGE)\
        .z_at(z)\
        .resolution(RESOLUTION)\
        .generate()
    bounds = observe_area.get_size()

    field = PressureFieldBuffer()

    focal_pos = z * Z_DIR
    foci = [focal_pos + np.array([50., 0, 0]), focal_pos - np.array([50., 0, 0])]
    amps = [1.0, 1.0]

    # # SMILE
    # radius = 45.0
    # num = 30
    # foci = []
    # for i in range(num):
    #     theta = 2 * math.pi * i / num
    #     foci.append(focal_pos + radius * np.array([math.cos(theta), math.sin(theta), 0.0]))
    # foci.append(focal_pos + np.array([radius * 0.3, radius * 0.3, 0]))
    # foci.append(focal_pos + np.array([-radius * 0.3, radius * 0.3, 0]))
    # for i in range(1, num // 4):
    #     theta = -math.pi * i / (num // 4)
    #     foci.append(focal_pos + radius * 0.6 * np.array([math.cos(theta), math.sin(theta), 0.0]))
    # amps = 1.0 * np.ones(len(foci))

    # IFFT
    # can only use when 2d phased array and 2d target
    for source in system.get_wave_sources():
        source.amp = 1.0
    path = os.path.join(os.path.dirname(__file__), '../acoustic-field-optimizer/examples/pattern/star.bmp')
    bottom_left = Vector3(np.array([0., 0., 0.]) - array_center)
    top_left = Vector3(np.array([0., TRANS_SIZE * (NUM_TRANS_Y - 1), 0.]) - array_center)
    bottom_right = Vector3(np.array([TRANS_SIZE * (NUM_TRANS_X - 1), 0., 0.]) - array_center)
    Optimizer.ifft(system, path, bottom_left, top_left, bottom_right, TRANS_SIZE, z)
    result = calculator.calculate(system, observe_area, field)
    plot(bounds, result, 'ifft')

    # GS-PAT
    # please specify maximum amplitude before
    for source in system.get_wave_sources():
        source.amp = 1.0
    Optimizer.gs_pat(system, foci, amps)
    result = calculator.calculate(system, observe_area, field)
    plot(bounds, result, 'gs-pat')

    # Gerchberg-Saxton
    # please specify maximum amplitude before
    for source in system.get_wave_sources():
        source.amp = 1.0
    Optimizer.gs(system, foci, amps)
    result = calculator.calculate(system, observe_area, field)
    plot(bounds, result, 'gs')

    # Naive linear synthesis
    # please specify maximum amplitude before
    for source in system.get_wave_sources():
        source.amp = 1.0
    Optimizer.naive(system, foci, amps)
    result = calculator.calculate(system, observe_area, field)
    plot(bounds, result, 'naive')

    # Horn
    # please specify maximum amplitude before
    for source in system.get_wave_sources():
        source.amp = 1.0
    Optimizer.horn(system, foci, amps)
    result = calculator.calculate(system, observe_area, field)
    plot(bounds, result, 'horn')

    # Long
    # please specify maximum amplitude before
    for source in system.get_wave_sources():
        source.amp = 1.0
    Optimizer.long2014(system, foci, amps)
    result = calculator.calculate(system, observe_area, field)
    plot(bounds, result, 'long')

    # Acoustic Power Optimization (with BFGS)
    for source in system.get_wave_sources():
        source.amp = 1.0
    Optimizer.apo(system, foci, amps, 2.0)
    result = calculator.calculate(system, observe_area, field)
    plot(bounds, result, 'apo')

    # Levenberg-Marquardt
    # Levenberg-Marquardt optimizer currently does not support amplitudes
    for source in system.get_wave_sources():
        source.amp = 1.0
    Optimizer.lm(system, foci, amps)
    result = calculator.calculate(system, observe_area, field)
    plot(bounds, result, 'lm')

    # Gauss-Newton
    # Gauss-Newton optimizer currently does not support amplitudes
    for source in system.get_wave_sources():
        source.amp = 1.0
    Optimizer.gauss_newton(system, foci, amps)
    result = calculator.calculate(system, observe_area, field)
    plot(bounds, result, 'gauss_newton')

    # Gradient-Descent
    # Gradient-Descent optimizer currently does not support amplitudes
    for source in system.get_wave_sources():
        source.amp = 1.0
    Optimizer.gradient_descent(system, foci, amps)
    result = calculator.calculate(system, observe_area, field)
    plot(bounds, result, 'gradient_descent')
