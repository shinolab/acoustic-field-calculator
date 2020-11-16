/*
 * File: gpu_prelude.rs
 * Project: src
 * Created Date: 19/09/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 25/09/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

use crate::Vector3;

pub use std::sync::Arc;

pub use vulkano::{
    buffer::{BufferUsage, CpuAccessibleBuffer, TypedBufferAccess},
    command_buffer::AutoCommandBufferBuilder,
    descriptor::{
        descriptor_set::{PersistentDescriptorSet, StdDescriptorPoolAlloc},
        pipeline_layout::PipelineLayout,
        PipelineLayoutAbstract,
    },
    device::{Device, DeviceExtensions, Queue},
    instance::{Instance, InstanceExtensions, PhysicalDevice},
    memory::pool::{PotentialDedicatedAllocation, StdMemoryPoolAlloc},
    pipeline::{self, shader::EntryPointAbstract, ComputePipeline},
    sync::{self, GpuFuture},
};

pub type PipeLine<T> = Arc<ComputePipeline<PipelineLayout<T>>>;
pub type GpuDevice = Arc<Device>;
pub type GpuQueue = Arc<Queue>;
pub type GpuBuffer<T> = Arc<CpuAccessibleBuffer<[T]>>;

pub fn to_four_multiple(x: usize) -> usize {
    ((x - 1) / 4 + 1) * 4
}

pub fn to_vec4(v: Vector3) -> [f32; 4] {
    [v[0] as f32, v[1] as f32, v[2] as f32, 0.]
}
