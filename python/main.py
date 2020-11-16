'''
File: main.py
Project: python
Created Date: 09/05/2020
Author: Shun Suzuki
-----
Last Modified: 28/09/2020
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2020 Hapis Lab. All rights reserved.

'''


import time
import math
import matplotlib.pyplot as plt
import numpy as np
import mpl_toolkits.axes_grid1

from afc import SphereWaveSource, T4010A1  # NOQA
from afc import CpuCalculator, GpuCalculator, GridAreaBuilder, PressureFieldBuffer, PowerFieldBuffer  # NOQA
from afc import plot_helper


if __name__ == '__main__':
    NUM_TRANS_X = 18
    NUM_TRANS_Y = 14
    TRANS_SIZE = 10.18

    FREQUENCY = 40e3
    SOUND_SPEED = 340e3
    WAVE_LENGTH = SOUND_SPEED / FREQUENCY
    Z_DIR = np.array([0, 0, 1])

    array_center = np.array([TRANS_SIZE * (NUM_TRANS_X - 1) / 2.0, TRANS_SIZE * (NUM_TRANS_Y - 1) / 2.0, 0])
    z = 150.0
    focal_pos = array_center + z * Z_DIR

    # Initialize calculator
    calculator = CpuCalculator(SOUND_SPEED)
    # calculator = GpuCalculator(SOUND_SPEED)

    # add wave source
    for y in range(NUM_TRANS_Y):
        for x in range(NUM_TRANS_X):
            pos = np.array([TRANS_SIZE * x, TRANS_SIZE * y, 0.])
            d = np.linalg.norm(pos - focal_pos)
            phase = (d % WAVE_LENGTH) / WAVE_LENGTH
            phase = -2.0 * math.pi * phase

            source = SphereWaveSource(pos, 1.0, phase, FREQUENCY)
            # source = T4010A1(pos, Z_DIR, 1.0, phase, FREQUENCY)
            calculator.add_wave_source(source)

    # # show phases
    # dpi = 72
    # fig = plt.figure(figsize=(6, 6), dpi=dpi)
    # axes = fig.add_subplot(111, aspect='equal')
    # scat = plot_helper.plot_phase_2d(fig, axes, calculator.get_wave_sources(), TRANS_SIZE)
    # plot_helper.add_colorbar(fig, axes, scat)
    # plt.savefig('phase.pdf')
    # plt.show()

    # Observe properties, units are mm
    R = 100.0
    X_RANGE = (array_center[0] - R / 2, array_center[0] + R / 2)
    Y_RANGE = (array_center[1] - R / 2, array_center[1] + R / 2)
    RESOLUTION = 1.0

    # Generate observe area, units are mm
    observe_area = GridAreaBuilder()\
        .x_range(X_RANGE)\
        .y_range(Y_RANGE)\
        .z_at(z)\
        .resolution(RESOLUTION)\
        .generate()

    # Generate target field buffer
    field = PressureFieldBuffer()

    # calculate acoustic pressure
    start = time.perf_counter_ns()
    result = calculator.calculate(observe_area, field)
    print("Elapsed: {0}".format((time.perf_counter_ns() - start) / 1000_000) + "[ms]")

    # plot field
    bounds = observe_area.get_size()
    array = result.reshape(bounds[0], bounds[1])
    DPI = 72
    fig = plt.figure(figsize=(6, 6), dpi=DPI)
    axes = fig.add_subplot(111, aspect='equal')
    heat_map = plot_helper.plot_acoustic_field_2d(axes, array, X_RANGE, Y_RANGE, RESOLUTION, ticks_step=10.0)
    divider = mpl_toolkits.axes_grid1.make_axes_locatable(axes)
    cax = divider.append_axes('right', '5%', pad='3%')
    fig.colorbar(heat_map, cax=cax)
    plt.savefig('xy.pdf')
    plt.show()
