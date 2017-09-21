use num::complex::*;
use std;

pub fn gen_sine(a: f32, f: f32, phi: f32, fs: f32, t: f32, res: &mut [f32]) {
    let size = (t * fs) as usize;
    if res.len() != size {
        panic!("Resulting array should be preallocated with size = {0}", size);
    }

    let mult = std::f32::consts::PI * 2f32 * f / fs;
    for k in 0..size {
        res[k] = a * (mult * (k as f32) + phi).cos();
    }
}

pub fn gen_complex_sine(a: f32, phi: f32, k: i32, n: usize, res: &mut [Complex<f32>]) {
    if res.len() != n {
        panic!("Resulting array should be preallocated with size = {0}", n);
    }

    let mult = std::f32::consts::PI * 2f32 * (k as f32) / n as f32;
    for j in 0..n {
        let alpha = (j as f32) * mult + phi;
        res[j].re = a * alpha.cos();
        res[j].im = -a * alpha.sin();
    }
}

pub fn dft(xs: &[f32], res: &mut [Complex<f32>]) {
    if res.len() != xs.len() {
        panic!("Resulting array should be preallocated with size = {0}", xs.len());
    }

    let size = xs.len();
    let mut sine = (0..size).map(|_| Complex::new(0f32, 0f32)).collect::<Vec<Complex<f32>>>();
    for k in 0..size {
        let mut x = Complex{re: 0f32, im: 0f32};
        gen_complex_sine(1f32, 0f32, k as i32, size, &mut sine);
        for j in 0..size {
            x += xs[j]*sine[j];
        }
        res[k] = x;
    }
}