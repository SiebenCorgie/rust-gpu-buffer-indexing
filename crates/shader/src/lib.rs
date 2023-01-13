#![cfg_attr(target_arch = "spirv", no_std)]
#![feature(asm_experimental_arch)]
// HACK(eddyb) can't easily see warnings otherwise from `spirv-builder` builds.
//#![deny(warnings)]

use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};

use shared::{BufTyOne, BufTyTwo, Push};
use spirv_std::glam::UVec3;

use spirv_std::macros::gpu_only;
//spirv macro
use spirv_std::{spirv, RuntimeArray};

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

#[spirv(typed_buffer)]
pub struct TypedBuffer<T: ?Sized> {
    // spooky! this field does not exist, so if it's referenced in rust code, things will explode
    _do_not_touch: u32,
    _phantom: PhantomData<T>,
}

impl<T> Deref for TypedBuffer<T> {
    type Target = T;
    #[gpu_only]
    fn deref(&self) -> &T {
        unsafe {
            core::arch::asm! {
                "%uint = OpTypeInt 32 0",
                "%uint_0 = OpConstant %uint 0",
                "%inner_ptr = OpAccessChain _ {buffer} %uint_0",
                "OpReturnValue %inner_ptr",
                buffer = in(reg) self,
                options(noreturn),
            }
        }
    }
}


impl<T> DerefMut for TypedBuffer<T> {
    #[gpu_only]
    fn deref_mut(&mut self) -> &mut T {
        unsafe {
            core::arch::asm! {
                "%uint = OpTypeInt 32 0",
                "%uint_0 = OpConstant %uint 0",
                "%inner_ptr = OpAccessChain _ {buffer} %uint_0",
                "OpReturnValue %inner_ptr",
                buffer = in(reg) self,
                options(noreturn),
            }
        }
    }
}

impl<T> Deref for TypedBuffer<[T]> {
    type Target = [T];
    #[gpu_only]
    fn deref(&self) -> &[T] {
        unsafe {
            core::arch::asm! {
                "%uint = OpTypeInt 32 0",
                "%uint_0 = OpConstant %uint 0",
                "%inner_ptr = OpAccessChain _ {buffer} %uint_0",
                "%inner_len = OpArrayLength %uint {buffer} 0",
                "%inner_slice_ptr = OpCompositeConstruct typeof*{dummy_ref_to_slice_ref} %inner_ptr %inner_len",
                "OpReturnValue %inner_slice_ptr",
                buffer = in(reg) self,
                dummy_ref_to_slice_ref = in(reg) &(&[] as &[T]),
                options(noreturn),
            }
        }
    }
}

impl<T> DerefMut for TypedBuffer<[T]> {
    #[gpu_only]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            core::arch::asm! {
                "%uint = OpTypeInt 32 0",
                "%uint_0 = OpConstant %uint 0",
                "%inner_ptr = OpAccessChain _ {buffer} %uint_0",
                "%inner_len = OpArrayLength %uint {buffer} 0",
                "%inner_slice_ptr = OpCompositeConstruct typeof*{dummy_ref_to_slice_ref} %inner_ptr %inner_len",
                "OpReturnValue %inner_slice_ptr",
                buffer = in(reg) self,
                dummy_ref_to_slice_ref = in(reg) &(&[] as &[T]),
                options(noreturn),
            }
        }
    }
}

#[spirv(compute(threads(64, 1, 1)))]
pub fn main(
    #[spirv(push_constant)] push: &Push,
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(descriptor_set = 0, binding = 0, storage_buffer)] buffers_a: &RuntimeArray<TypedBuffer<[u32]>>,
    #[spirv(descriptor_set = 0, binding = 0, storage_buffer)] buffers_b: &mut RuntimeArray<TypedBuffer<[u32]>>
) {
    let widx = id.x;
    if widx > push.size{
        return;
    }

    //load
    let mut a = unsafe{
        buffers_a.index(push.src_hdl.index() as usize)[widx as usize]
    };

    //store
    unsafe{
        buffers_b.index_mut(push.dst_hdl.index() as usize)[widx as usize] = a;
    }

}
