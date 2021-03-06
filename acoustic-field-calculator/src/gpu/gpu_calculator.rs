/*
 * File: gpu_calculator.rs
 * Project: calculator
 * Created Date: 18/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use crate::{calculator::FieldCalculator, core::wave_sources::WaveSource, gpu::gpu_prelude::*, gpu::*};

/// GPU Calculator
pub struct GpuCalculator {
    device: Arc<Device>,
    queue: Arc<Queue>,
}

impl Default for GpuCalculator {
    fn default() -> Self {
        Self::new()
    }
}

impl GpuCalculator {
    pub fn new() -> Self {
        let (device, queue) = Self::init_gpu();
        Self { device, queue }
    }

    fn init_gpu() -> (Arc<Device>, Arc<Queue>) {
        let instance = Instance::new(None, &InstanceExtensions::none(), None).unwrap();

        let physical = PhysicalDevice::enumerate(&instance).next().unwrap();
        let queue_family = physical
            .queue_families()
            .find(|&q| q.supports_compute())
            .unwrap();
        let (device, mut queues) = Device::new(
            physical,
            physical.supported_features(),
            &DeviceExtensions {
                khr_storage_buffer_storage_class: true,
                ..DeviceExtensions::none()
            },
            [(queue_family, 0.5)].iter().cloned(),
        )
        .unwrap();
        let queue = queues.next().unwrap();

        (device, queue)
    }
}

impl<S, M, A, O, F> FieldCalculator<S, M, A, O, F> for GpuCalculator
where
    S: WaveSource,
    M: GpuPropagationMedium<S>,
    A: SizedArea,
    F: GpuFieldBuffer<O>,
{
    fn calculate(&self, medium: &M, observe_area: &A, buffer: &mut F) {
        let device = self.device.clone();
        let queue = self.queue.clone();
        F::calculate_field(medium, observe_area, buffer, device, queue);
    }
}
