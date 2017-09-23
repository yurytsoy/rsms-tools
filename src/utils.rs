use num::complex::*;
use std;

/// Computes real sine at descrete time steps:
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

/// Compute complex sine at discrete time steps:
///    a*exp(-i * 2 * pi * k * t / n), for t in [0, n-1]
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

/// Compute DFT for a given array.
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

/// Compute inverse DFT.
pub fn idft(xs: &[Complex<f32>], res: &mut [Complex<f32>]) {
    if res.len() != xs.len() {
        panic!("Resulting array should be preallocated with size = {0}", xs.len());
    }

    let size = xs.len();
    let mut sine = (0..size).map(|_| Complex::new(0f32, 0f32)).collect::<Vec<Complex<f32>>>();
    for k in 0..size {
        let mut x = Complex{re: 0f32, im: 0f32};
        // take negative k to compute inverse.
        gen_complex_sine(1f32, 0f32, -(k as i32), size, &mut sine);
        for j in 0..size {
            x += xs[j]*sine[j];
        }
        res[k] = x / (size as f32);
    }
}

pub fn gen_mag_spec(xs: &[f32], res: &mut [f32]) {
    if res.len() != xs.len() {
        panic!("Resulting array should be preallocated with size = {0}", xs.len());
    }

    let mut dft_res = (0..xs.len()).map(|_| Complex::new(0f32, 0f32)).collect::<Vec<Complex<f32>>>();
    dft(xs, &mut dft_res);
    for k in 0..xs.len() {
        res[k] = dft_res[k].norm();
    }
}