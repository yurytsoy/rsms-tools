extern crate num;

mod utils;

/// The tests are numbered according to the assignments in the course "Audio Signal Processing for Music Applications".
#[cfg(test)]
mod tests {
    use num::complex::*;
    use utils;

    #[test]
    fn test_a2_1() {
        let mut res = vec![0f32; 5];
        utils::gen_sine(1f32, 10f32, 1f32, 50f32, 0.1f32, &mut res);
        assert_eq!(res, [0.5403023f32, -0.6333239f32, -0.93171793f32, 0.05749059f32, 0.9672491f32]);
    }

    #[test]
    fn test_a2_2() {
        // let mut res = vec![Complex<f32>; 5];
        let mut res = (0..5).map(|_| Complex::<f32>::new(0f32, 0f32)).collect::<Vec<Complex<f32>>>();
        utils::gen_complex_sine(1f32, 0f32, 1i32, 5usize, &mut res);
        // println!("{:?}", res);
        assert_eq!(res, [Complex{re: 1f32, im: 0f32}, Complex{re: 0.30901697, im: -0.95105654 }, Complex { re: -0.80901706, im: -0.5877852 }, Complex { re: -0.80901694, im: 0.58778536 }, Complex { re: 0.30901712, im: 0.9510565 }]);
    }

    #[test]
    fn test_a2_3() {
        let xs = vec![1f32, 2f32, 3f32, 4f32];
        let mut res = (0..xs.len()).map(|_| Complex::new(0f32, 0f32)).collect::<Vec<Complex<f32>>>();
        utils::dft(&xs, &mut res);
        assert_eq!(res, [Complex { re: 10f32, im: 0f32 }, Complex { re: -2f32, im: 2.0000002 }, Complex { re: -2f32, im: -0.00000025429205 }, Complex { re: -2.0000002, im: -2f32 }]);
    }

    #[test]
    fn test_a2_4() {
        let xs = vec![-1f32, 3f32, 1f32, 0f32];
        let mut res_dft = (0..xs.len()).map(|_| Complex::new(0f32, 0f32)).collect::<Vec<Complex<f32>>>();
        utils::dft(&xs, &mut res_dft);
        let mut res_idft = (0..xs.len()).map(|_| Complex::new(0f32, 0f32)).collect::<Vec<Complex<f32>>>();
        utils::idft(&res_dft, &mut res_idft);
        assert!(res_idft.iter().zip(xs.iter())
                        .map(|(x, x0)| (x-x0).norm())
                        .all(|x| x < 1e-7));
    }

    #[test]
    fn test_a2_5() {
        let xs = vec![1f32, 2f32, 3f32, 4f32];
        let mut res_spec = [0f32; 4];
        utils::gen_mag_spec(&xs, &mut res_spec);
        assert_eq!(res_spec, [10f32, 2.8284273f32, 2f32, 2.8284273f32]);
    }
}
