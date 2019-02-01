#![feature(stdsimd)]
#![feature(test)]
extern crate test;
extern crate itertools;
extern crate faster;
use faster::*;
use std::slice;

#[no_mangle]
pub unsafe extern "C" fn rust_32f_x2_multiply_32f(
    cvec: *mut f32,
    avec: *const f32,
    bvec: *const f32,
    points: u32,
) {
    let buffa: &[f32] = slice::from_raw_parts(avec, points as usize);
    let buffb: &[f32] = slice::from_raw_parts(bvec, points as usize);
    let buffc: &mut [f32] = slice::from_raw_parts_mut(cvec, points as usize);
    for i in 0..points as usize {
        buffc[i] = buffa[i] * buffb[i];
    }
}

#[no_mangle]
pub unsafe extern "C" fn rust_f_32f_x2_multiply_32f(
    cvec: *mut f32,
    avec: *const f32,
    bvec: *const f32,
    points: u32,
) {
    let buffa: &[f32] = slice::from_raw_parts(avec, points as usize);
    let buffb: &[f32] = slice::from_raw_parts(bvec, points as usize);
    let buffc: &mut [f32] = slice::from_raw_parts_mut(cvec, points as usize);
    (buffa.simd_iter(f32s(0.0)), buffb.simd_iter(f32s(0.0)))
        .zip()
        .simd_map(|(a, b)| a * b)
        .scalar_fill(buffc);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn mult() {
        const _ARRAY_LEN: usize = 10000;
        let a: [f32; _ARRAY_LEN] = [2.0; _ARRAY_LEN];
        let b: [f32; _ARRAY_LEN] = [3.0; _ARRAY_LEN];
        let c = &mut [0.0; _ARRAY_LEN];
        let c_exp: [f32; _ARRAY_LEN] = [6.0; _ARRAY_LEN];

        unsafe
        {
            rust_32f_x2_multiply_32f(c.as_mut_ptr(), a.as_ptr(), b.as_ptr(), _ARRAY_LEN as u32);
        }
        itertools::assert_equal((*c).iter(), c_exp.iter());
    }

    #[test]
    fn mult_faster() {
        const _ARRAY_LEN: usize = 10000;
        let a: [f32; _ARRAY_LEN] = [2.0; _ARRAY_LEN];
        let b: [f32; _ARRAY_LEN] = [3.0; _ARRAY_LEN];
        let c = &mut [0.0; _ARRAY_LEN];
        let c_exp: [f32; _ARRAY_LEN] = [6.0; _ARRAY_LEN];

        unsafe
        {
            rust_f_32f_x2_multiply_32f(c.as_mut_ptr(), a.as_ptr(), b.as_ptr(), _ARRAY_LEN as u32);
        }
        itertools::assert_equal((*c).iter(), c_exp.iter());
    }

    #[bench]
    fn bench_mult(b: &mut Bencher) {
        b.iter(|| mult());
    }

    #[bench]
    fn bench_mult_faster(b: &mut Bencher) {
        b.iter(|| mult_faster());
    }
}

