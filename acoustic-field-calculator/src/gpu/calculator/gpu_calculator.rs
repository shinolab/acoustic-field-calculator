/*
 * File: gpu_calculator.rs
 * Project: calculator
 * Created Date: 18/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 25/09/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use crate::gpu::gpu_prelude::*;
use crate::gpu::*;

use crate::calculator::WaveSourceContainer;
use crate::core::Float;

/// GPU Calculator
pub struct GpuCalculator<S: GpuWaveSource> {
    sources: Vec<S>,
    sound_speed: Float,
    device: Arc<Device>,
    queue: Arc<Queue>,
}

impl<S: GpuWaveSource> GpuCalculator<S> {
    pub(crate) fn new(sound_speed: Float) -> Self {
        let (device, queue) = Self::init_gpu();
        Self {
            sources: vec![],
            sound_speed,
            device,
            queue,
        }
    }
}

impl<S: GpuWaveSource> GpuCalculator<S> {
    /// Calculate field at observe area
    ///
    /// # Arguments
    ///
    /// * `observe_area` - Observed area to calculate
    /// * `field` - Field to calculate
    ///
    pub fn calculate<A: SizedArea, T: GpuFieldBuffer>(&mut self, observe_area: &A, field: &mut T) {
        let device = self.device.clone();
        let queue = self.queue.clone();
        field.calculate_field(self, observe_area, device, queue);
    }
}

impl<S: GpuWaveSource> WaveSourceContainer<S> for GpuCalculator<S> {
    fn wave_sources(&self) -> &[S] {
        &self.sources
    }

    fn wave_sources_mut(&mut self) -> &mut Vec<S> {
        &mut self.sources
    }

    fn sound_speed(&self) -> Float {
        self.sound_speed
    }
}

impl<S: GpuWaveSource> GpuCalculator<S> {
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
