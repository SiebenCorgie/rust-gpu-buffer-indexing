#![cfg_attr(target_arch = "spirv", no_std)]
#![feature(asm_experimental_arch)]
// HACK(eddyb) can't easily see warnings otherwise from `spirv-builder` builds.
//#![deny(warnings)]

use core::marker::PhantomData;

use shared::{BufTyOne, BufTyTwo, Push};
use spirv_std::glam::UVec3;

use spirv_std::macros::gpu_only;
//spirv macro
use spirv_std::{spirv, RuntimeArray};

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

#[spirv(typed_buffer)]
pub struct TypedBuffer<T: Sized + 'static>{
    //borrowing the hidden data trick
    data: u32,
    dataty: PhantomData<T>
}
impl<T: Sized + 'static> TypedBuffer<T>{

    #[gpu_only]
    pub unsafe fn access(&self) -> &T {
        core::arch::asm! {
            "%result = OpAccessChain _ {arr} 0",
            "OpReturnValue %result",
            arr = in(reg) self,
            options(noreturn),
        }
    }

    #[gpu_only]
    pub unsafe fn access_mut(&mut self) -> &mut T {
        core::arch::asm! {
            "%result = OpAccessChain _ {arr} 0",
            "OpReturnValue %result",
            arr = in(reg) self,
            options(noreturn),
        }
    }
}

#[spirv(compute(threads(64, 1, 1)))]
pub fn main(
    #[spirv(push_constant)] push: &Push,
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(descriptor_set = 0, binding = 0, storage_buffer)] buffers_a: &RuntimeArray<TypedBuffer<RuntimeArray<u32>>>,
    #[spirv(descriptor_set = 0, binding = 0, storage_buffer)] buffers_b: &mut RuntimeArray<TypedBuffer<RuntimeArray<u32>>>
) {
    let widx = id.x;
    if widx > push.size{
        return;
    }

    let a = unsafe{
        *buffers_a.index(push.src_hdl.index() as usize)
                 .access()
                 .index(widx as usize)
    };

    //store
    unsafe{
        *buffers_b.index_mut(push.dst_hdl.index() as usize)
                  .access_mut()
                  .index_mut(widx as usize) = a;
    }
}
