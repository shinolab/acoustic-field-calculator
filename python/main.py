'''
File: main.py
Project: python
Created Date: 09/05/2020
Author: Shun Suzuki
-----
Last Modified: 19/11/2020
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2020 Hapis Lab. All rights reserved.

'''


import time
import math
import matplotlib.pyplot as plt
import numpy as np
import mpl_toolkits.axes_grid1

from afc import UniformSystem
from afc import SphereWaveSource, T4010A1  # NOQA
from afc import CpuCalculator, GpuCalculator, GridAreaBuilder, PressureFieldBuffer, PowerFieldBuffer  # NOQA
from afc import plot_helper


if __name__ == '__main__':
    NUM_TRANS_X = 18
    NUM_TRANS_Y = 14
    TRANS_SIZE = 10.18
    FREQUENCY = 40e3
    TEMPERATURE = 300
    Z_DIR = np.array([0, 0, 1])

    array_center = np.array([TRANS_SIZE * (NUM_TRANS_X - 1) / 2.0, TRANS_SIZE * (NUM_TRANS_Y - 1) / 2.0, 0])
    z = 150.0
    focal_pos = array_center + z * Z_DIR

    # Initialize calculator
    calculator = CpuCalculator()
    # calculator = GpuCalculator()

    # init system
    system = UniformSystem(TEMPERATURE)
    # add wave source
    for y in range(NUM_TRANS_Y):
        for x in range(NUM_TRANS_X):
            pos = np.array([TRANS_SIZE * x, TRANS_SIZE * y, 0.])
            # source = SphereWaveSource(pos, 1.0, 0.0, FREQUENCY)
            source = T4010A1(pos, Z_DIR, 1.0, 0.0, FREQUENCY)
            system.add_wave_source(source)

    system.info()
    system.info_of_source(0)

    # set phase to produce focus
    sound_speed = system.sound_speed()
    for source in system.get_wave_sources():
        d = np.linalg.norm(source.pos - focal_pos)
        wavelength = sound_speed / FREQUENCY
        phase = (d % wavelength) / wavelength
        phase = -2.0 * math.pi * phase
        source.phase = phase

    # # show phases
    # dpi = 72
    # fig = plt.figure(figsize=(6, 6), dpi=dpi)
    # axes = fig.add_subplot(111, aspect='equal')
    # scat = plot_helper.plot_phase_2d(fig, axes, system.get_wave_sources(), TRANS_SIZE)
    # plot_helper.add_colorbar(fig, axes, scat)
    # plt.savefig('phase.png')
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
    result = calculator.calculate(system, observe_area, field)
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
    plt.savefig('xy.png')
    plt.show()
