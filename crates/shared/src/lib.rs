#![cfg_attr(target_arch = "spirv", no_std)]

use marpii_rmg_task_shared::ResourceHandle;

#[cfg(not(target_arch = "spirv"))]
use bytemuck::{Pod, Zeroable};

#[cfg_attr(
    not(target_arch = "spirv"),
    derive(Clone, Copy, Hash, PartialEq, PartialOrd, Eq, Debug, Pod, Zeroable)
)]
#[cfg_attr(target_arch = "spirv", derive(Clone, Copy))]
#[repr(C)]
pub struct BufTyOne{
    pub a: f32,
    pub b: f32,
    pub pad: [f32; 2],
}

pub struct BufTyTwo{
    pub new: [u32; 4],
    pub a: f32,
    pub b: f32,
    pub pad: [f32; 2]
}

pub struct Push{
    src_hdl: ResourceHandle,
    dst_hdl: ResourceHandle,
    size: u32,
    pad: u32
}
