/*
 * File: gpu_cache.rs
 * Project: field_buffer
 * Created Date: 20/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/11/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

macro_rules! gen_cache {
    ($shader_mod: ident) => {
        use once_cell::sync::Lazy;
        use std::sync::atomic::{AtomicPtr, Ordering};

        struct GpuCache {
            device: Option<GpuDevice>,
            obs_points: (AtomicPtr<Vector3>, usize),
            pub pipeline: Option<PipeLine<$shader_mod::Layout>>,
            pub results_buf: Option<GpuBuffer<f32>>,
            pub obs_points_buf: Option<GpuBuffer<[f32; 4]>>,
        }

        static GPU_CACHE: Lazy<MutStatic<GpuCache>> = Lazy::new(|| {
            let cache = MutStatic::new();
            cache.set(GpuCache::new()).unwrap();
            cache
        });

        impl GpuCache {
            pub fn new() -> Self {
                Self {
                    device: None,
                    obs_points: (AtomicPtr::new(std::ptr::null_mut()), 0),
                    pipeline: None,
                    results_buf: None,
                    obs_points_buf: None,
                }
            }

            pub fn update_cache<T, F: GpuFieldType<T>, A: SizedArea<T, F>>(
                &mut self,
                len: usize,
                device: GpuDevice,
                observe_area: &A,
            ) {
                if self.device.is_none() {
                    self.initialize_cache(len, device.clone(), observe_area);
                    return;
                }

                let current_device = self.device.clone().unwrap();
                if Arc::ptr_eq(&current_device, &device) {
                    self.update(len, device, observe_area);
                } else {
                    self.initialize_cache(len, device, observe_area);
                }
            }

            fn initialize_cache<T, F: GpuFieldType<T>, A: SizedArea<T, F>>(
                &mut self,
                len: usize,
                device: GpuDevice,
                observe_area: &A,
            ) {
                self.init_pipeline(device.clone());
                self.init_observe_area(observe_area, len, device.clone());
                self.device = Some(device);
            }

            fn update<T, F: GpuFieldType<T>, A: SizedArea<T, F>>(
                &mut self,
                len: usize,
                device: GpuDevice,
                observe_area: &A,
            ) {
                let obs_points = observe_area.points();
                let (current_obs_p, current_obs_len) = &self.obs_points;
                let current_obs_p = current_obs_p.load(Ordering::Acquire);
                if (*current_obs_len != obs_points.len())
                    || !std::ptr::eq(current_obs_p, obs_points.as_ptr())
                {
                    self.init_observe_area(observe_area, len, device);
                }
            }

            fn init_pipeline(&mut self, device: GpuDevice) {
                let pipeline = std::sync::Arc::new({
                    let shader = $shader_mod::Shader::load(device.clone()).unwrap();
                    vulkano::pipeline::ComputePipeline::new(
                        device.clone(),
                        &shader.main_entry_point(),
                        &(),
                    )
                    .unwrap()
                });
                self.pipeline = Some(pipeline);
            }

            fn init_observe_area<T, F: GpuFieldType<T>, A: SizedArea<T, F>>(
                &mut self,
                observe_area: &A,
                len: usize,
                device: GpuDevice,
            ) {
                let obs_points = observe_area.points();
                let res_buffer = {
                    let data_iter = (0..len).map(|_| f32::default());
                    CpuAccessibleBuffer::from_iter(
                        device.clone(),
                        BufferUsage::all(),
                        false,
                        data_iter,
                    )
                    .unwrap()
                };
                let obs_buffer = {
                    let pos = (0..to_four_multiple(len)).map(|n| {
                        if n < obs_points.len() {
                            to_vec4(obs_points[n])
                        } else {
                            Default::default()
                        }
                    });
                    CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false, pos)
                        .unwrap()
                };
                self.results_buf = Some(res_buffer);
                self.obs_points_buf = Some(obs_buffer);
                self.obs_points = (
                    AtomicPtr::new(obs_points.as_ptr() as *mut _),
                    obs_points.len(),
                );
            }
        }
    };
}
