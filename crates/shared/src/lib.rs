#![cfg_attr(target_arch = "spirv", no_std)]

use marpii_rmg_task_shared::ResourceHandle;

#[cfg(not(target_arch = "spirv"))]
use bytemuck::{Pod, Zeroable};

#[cfg_attr(
    not(target_arch = "spirv"),
    derive(Clone, Copy, PartialEq, PartialOrd, Debug, Pod, Zeroable)
)]
#[cfg_attr(target_arch = "spirv", derive(Clone, Copy))]
#[repr(C, align(16))]
pub struct BufTyOne{
    pub a: f32,
    pub b: f32,
    pub pad: [f32; 2],
}

#[cfg_attr(
    not(target_arch = "spirv"),
    derive(Clone, Copy, PartialEq, PartialOrd, Debug, Pod, Zeroable)
)]
#[cfg_attr(target_arch = "spirv", derive(Clone, Copy))]
#[repr(C, align(16))]
pub struct BufTyTwo{
    pub new: [u32; 4],
    pub a: f32,
    pub b: f32,
    pub pad: [f32; 2]
}

#[cfg_attr(
    not(target_arch = "spirv"),
    derive(Clone, Copy, PartialEq, PartialOrd, Debug, Pod, Zeroable)
)]
#[cfg_attr(target_arch = "spirv", derive(Clone, Copy))]
#[repr(C, align(16))]
pub struct Push{
    pub src_hdl: ResourceHandle,
    pub dst_hdl: ResourceHandle,
    pub size: u32,
    pub pad: u32
}
