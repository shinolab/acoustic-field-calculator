/*
 * File: pressure_field.rs
 * Project: field_buffer
 * Created Date: 18/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 02/10/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */
use super::traits::*;
use crate::field_buffer::PowerField;
use crate::gpu::gpu_prelude::*;
use crate::gpu::*;
use crate::Vector3;
use crate::{calculator::*, field_buffer::FieldBuffer};

use mut_static::MutStatic;

mod cs_power {
    vulkano_shaders::shader! {
        ty: "compute",
        path: "./assets/shaders/power.comp"
    }
}

gen_cache!(cs_power);

#[repr(C)]
struct Config {
    source_num: u32,
    num_x: u32,
    num_y: u32,
    num_z: u32,
    directivity_num: u32,
    _dummy1: u32,
    _dummy2: u32,
    _dummy3: u32,
}

impl GpuFieldBuffer for PowerField<f32> {
    fn calculate_field<S: GpuWaveSource, F: SizedArea>(
        &mut self,
        calculator: &mut GpuCalculator<S>,
        observe_area: &F,
        device: GpuDevice,
        queue: GpuQueue,
    ) {
        let len = observe_area.observe_points().len();
        GPU_CACHE
            .write()
            .unwrap()
            .update_cache(len, device.clone(), observe_area);

        let sources = calculator.wave_sources();
        let directivity = S::directivity();
        let directivity_len = directivity.len();
        let (num_x, num_y, num_z) = observe_area.size();
        let config_buffer = {
            let source_num = sources.len() as u32;
            let config = Config {
                source_num,
                num_x,
                num_y,
                num_z,
                directivity_num: directivity.len() as u32,
                _dummy1: 0,
                _dummy2: 0,
                _dummy3: 0,
            };
            CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), false, config)
                .unwrap()
        };
        let directivity_buffer = {
            let dir = (0..to_four_multiple(directivity_len)).map(|n| {
                if n < directivity_len {
                    directivity[n] as f32
                } else {
                    0.
                }
            });
            CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false, dir).unwrap()
        };

        let pipeline = GPU_CACHE.read().unwrap().pipeline.clone().unwrap();
        let res_buffer = GPU_CACHE.read().unwrap().results_buf.clone().unwrap();

        let source_pos_buffer = {
            let pos = (0..to_four_multiple(sources.len())).map(|n| {
                if n < sources.len() {
                    to_vec4(sources[n].position())
                } else {
                    Default::default()
                }
            });
            CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false, pos).unwrap()
        };
        let source_dir_buffer = {
            let pos = (0..to_four_multiple(sources.len())).map(|n| {
                if n < sources.len() {
                    to_vec4(sources[n].direction())
                } else {
                    Default::default()
                }
            });
            CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false, pos).unwrap()
        };

        let wavenum_buffer = {
            let pos = (0..to_four_multiple(sources.len())).map(|n| {
                if n < sources.len() {
                    sources[n].wavenumber() as f32
                } else {
                    Default::default()
                }
            });
            CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false, pos).unwrap()
        };
        let atten_buffer = {
            let pos = (0..to_four_multiple(sources.len())).map(|n| {
                if n < sources.len() {
                    sources[n].attenuation() as f32
                } else {
                    Default::default()
                }
            });
            CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false, pos).unwrap()
        };

        let source_phase_buffer = {
            let pos = (0..to_four_multiple(sources.len())).map(|n| {
                if n < sources.len() {
                    sources[n].phase() as f32
                } else {
                    Default::default()
                }
            });
            CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false, pos).unwrap()
        };
        let source_amp_buffer = {
            let pos = (0..to_four_multiple(sources.len())).map(|n| {
                if n < sources.len() {
                    sources[n].amp() as f32
                } else {
                    Default::default()
                }
            });
            CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false, pos).unwrap()
        };

        let obs_buffer = GPU_CACHE.read().unwrap().obs_points_buf.clone().unwrap();

        let set_0 = Arc::new(
            PersistentDescriptorSet::start(
                pipeline.layout().descriptor_set_layout(0).unwrap().clone(),
            )
            .add_buffer(res_buffer.clone())
            .unwrap()
            .build()
            .unwrap(),
        );

        let set_1 = Arc::new(
            PersistentDescriptorSet::start(
                pipeline.layout().descriptor_set_layout(1).unwrap().clone(),
            )
            .add_buffer(config_buffer)
            .unwrap()
            .add_buffer(directivity_buffer)
            .unwrap()
            .build()
            .unwrap(),
        );

        let set_2 = Arc::new(
            PersistentDescriptorSet::start(
                pipeline.layout().descriptor_set_layout(2).unwrap().clone(),
            )
            .add_buffer(source_pos_buffer)
            .unwrap()
            .add_buffer(source_dir_buffer)
            .unwrap()
            .add_buffer(wavenum_buffer)
            .unwrap()
            .add_buffer(atten_buffer)
            .unwrap()
            .build()
            .unwrap(),
        );

        let set_3 = Arc::new(
            PersistentDescriptorSet::start(
                pipeline.layout().descriptor_set_layout(3).unwrap().clone(),
            )
            .add_buffer(source_phase_buffer)
            .unwrap()
            .add_buffer(source_amp_buffer)
            .unwrap()
            .build()
            .unwrap(),
        );

        let set_4 = Arc::new(
            PersistentDescriptorSet::start(
                pipeline.layout().descriptor_set_layout(4).unwrap().clone(),
            )
            .add_buffer(obs_buffer)
            .unwrap()
            .build()
            .unwrap(),
        );

        let mut builder =
            AutoCommandBufferBuilder::primary_one_time_submit(device.clone(), queue.family())
                .unwrap();
        builder
            .dispatch(
                [num_x, num_y, num_z],
                pipeline,
                (set_0, set_1, set_2, set_3, set_4),
                (),
            )
            .unwrap();
        let command_buffer = builder.build().unwrap();

        let future = sync::now(device)
            .then_execute(queue, command_buffer)
            .unwrap()
            .then_signal_fence_and_flush()
            .unwrap();
        future.wait(None).unwrap();

        let data_buffer_content = res_buffer.read().unwrap();

        let buffer = self.buffer_mut();
        buffer.resize(observe_area.observe_points().len(), Default::default());
        buffer[0..len].copy_from_slice(&data_buffer_content[0..len]);
    }
}
