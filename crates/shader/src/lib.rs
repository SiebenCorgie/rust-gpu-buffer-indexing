#![cfg_attr(target_arch = "spirv", no_std)]
// HACK(eddyb) can't easily see warnings otherwise from `spirv-builder` builds.
//#![deny(warnings)]

use spirv_std::glam::UVec3;

//spirv macro
use spirv_std::spirv;

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

#[spirv(compute(threads(64, 1, 1)))]
pub fn main(
    #[spirv(global_invocation_id)] id: UVec3,
) {

    let idx = id.x;
}
