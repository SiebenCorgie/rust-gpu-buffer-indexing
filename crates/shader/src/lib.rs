#![cfg_attr(target_arch = "spirv", no_std)]
// HACK(eddyb) can't easily see warnings otherwise from `spirv-builder` builds.
//#![deny(warnings)]

use shared::{BufTyOne, BufTyTwo, Push};
use spirv_std::glam::UVec3;

use spirv_std::macros::gpu_only;
//spirv macro
use spirv_std::{spirv, RuntimeArray};

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;


pub struct TypedBuffer<T>(pub T);
impl<T> TypedBuffer<T>{

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
    #[spirv(set = 0, binding = 0, typed_buffer)] buffers_a: &RuntimeArray<TypedBuffer<RuntimeArray<BufTyOne>>>,
    #[spirv(set = 0, binding = 0, typed_buffer)] buffers_b: &mut RuntimeArray<TypedBuffer<RuntimeArray<BufTyTwo>>>
) {
    let widx = id.x;
    if widx > push.size{
        return;
    }

    let a = unsafe{
        buffers_a.index(push.src_hdl.index() as usize)
            .access()
            .index(widx as usize)
    };

    let b = BufTyTwo{
        new: [4,3,2, widx],
        a: a.a,
        b: a.b,
        pad: a.pad
    };

    //store
    unsafe{
        *buffers_b.index_mut(push.dst_hdl.index() as usize)
            .access_mut()
            .index_mut(widx as usize) = b;
    }
}
