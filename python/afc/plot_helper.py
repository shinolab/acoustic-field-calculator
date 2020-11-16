'''
File: plot_helper.py
Project: afc
Created Date: 09/05/2020
Author: Shun Suzuki
-----
Last Modified: 01/10/2020
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2020 Hapis Lab. All rights reserved.

'''

import math
import numpy as np
import mpl_toolkits.axes_grid1


def adjust_marker_size(fig, axes, scat, radius):
    """
    set size proportional to axis
    https://stackoverflow.com/questions/36458458/python-scatter-plot-area-size-proportional-axis-length
    """
    fig.canvas.draw()

    r_pix = axes.transData.transform((radius, radius)) - axes.transData.transform((0, 0))
    sizes = (2.0 * r_pix * 72 / fig.dpi)**2

    scat.set_sizes(sizes)


def add_colorbar(fig, axes, mappable, position='right', size='5%', pad='3%'):
    divider = mpl_toolkits.axes_grid1.make_axes_locatable(axes)
    cax = divider.append_axes(position, size, pad=pad)
    fig.colorbar(mappable, cax=cax)


def plot_phase_2d(fig, axes, transducers, trans_size, cmap='jet', marker='o'):
    trans_x = list(map(lambda s: s.pos[0], transducers))
    trans_y = list(map(lambda s: s.pos[1], transducers))
    trans_phase = list(map(lambda s: s.phase - math.pi, transducers))

    scat = axes.scatter(trans_x, trans_y, c=trans_phase, cmap=cmap, s=0,
                        marker=marker, vmin=-math.pi, vmax=math.pi,
                        clip_on=False, linewidths=0)

    add_colorbar(fig, axes, scat)
    adjust_marker_size(fig, axes, scat, trans_size / 2)

    return scat


def plot_acoustic_field_2d(axes, acoustic_pressures_2d, observe_x, observe_y, resolution, ticks_step, cmap='jet'):
    heatmap = axes.pcolor(abs(acoustic_pressures_2d), cmap=cmap)
    x_label_num = int(math.floor((observe_x[1] - observe_x[0]) / ticks_step)) + 1
    y_label_num = int(math.floor((observe_y[1] - observe_y[0]) / ticks_step)) + 1
    x_labels = ['{:.2f}'.format(observe_x[0] + ticks_step * i) for i in range(x_label_num)]
    y_labels = ['{:.2f}'.format(observe_y[0] + ticks_step * i) for i in range(y_label_num)]
    x_ticks = [ticks_step / resolution * i for i in range(x_label_num)]
    y_ticks = [ticks_step / resolution * i for i in range(y_label_num)]
    axes.set_xticks(np.array(x_ticks) + 0.5, minor=False)
    axes.set_yticks(np.array(y_ticks) + 0.5, minor=False)
    axes.set_xticklabels(x_labels, minor=False)
    axes.set_yticklabels(y_labels, minor=False)

    return heatmap
