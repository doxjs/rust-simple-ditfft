
use num_complex::Complex;
use std::f64::consts::{ PI };

pub type ComplexVec = Vec<Complex<f64>>;

pub struct Factor;

impl Factor {
    pub fn new(r: i32, n: i32) -> Complex<f64> {
        let r = r as f64;
        let n = n as f64;
        let theta = (-2.0_f64 * r * PI / n) as f64;

        Complex::new(theta.cos(), theta.sin())
    }
}

pub fn dit_fftf(hn: &Vec<f64>) -> ComplexVec {

    let hn_len = hn.len();
    let level = (hn_len as f64).log2().ceil() as i32;
    let fetched_len = 2.0_f64.powi(level) as usize;
    let half_fetched_len = fetched_len / 2;

    let mut fft_results: ComplexVec = Vec::with_capacity(fetched_len);

    if hn_len > 2 {

        let mut even_hn: Vec<f64> = vec![];
        let mut odd_hn: Vec<f64> = vec![];

        let even_fft: ComplexVec;
        let odd_fft: ComplexVec;

        let mut factors: ComplexVec = vec![];

        for r in 0..half_fetched_len {
            let odd_index = 2 * r + 1;
            let even_index = 2 * r;

            if even_index < hn_len {
                even_hn.push(hn[even_index]);
            } else {
                even_hn.push(0.0);
            }

            if odd_index < hn_len {
                odd_hn.push(hn[odd_index]);
            } else {
                odd_hn.push(0.0);
            }
        }

        even_fft = dit_fftf(&even_hn);
        odd_fft = dit_fftf(&odd_hn);

        for r in 0..half_fetched_len {
            let factor = Factor::new(r as i32, fetched_len as i32);

            factors.push(factor);
            fft_results.push(even_fft[r] + factor * odd_fft[r]);
        }

        for r in 0..half_fetched_len {
            fft_results.push(even_fft[r] - factors[r] * odd_fft[r]);
        }
    } else {
        fft_results.push(Complex::new(hn[0] + hn[1], 0.0));
        fft_results.push(Complex::new(hn[0] - hn[1], 0.0));
    }

    fft_results
}

pub fn dit_ffti(hn: &Vec<i32>) -> ComplexVec {

    let hn_len = hn.len();
    let level = (hn_len as f64).log2().ceil() as i32;
    let fetched_len = 2.0_f64.powi(level) as usize;
    let half_fetched_len = fetched_len / 2;

    let mut fft_results: ComplexVec = vec![];

    if hn_len > 2 {

        let mut even_hn: Vec<i32> = vec![];
        let mut odd_hn: Vec<i32> = vec![];

        let even_fft: ComplexVec;
        let odd_fft: ComplexVec;

        let mut factors: ComplexVec = vec![];

        for r in 0..half_fetched_len {
            let odd_index = 2 * r + 1;
            let even_index = 2 * r;

            if even_index < hn_len {
                even_hn.push(hn[even_index]);
            } else {
                even_hn.push(0);
            }

            if odd_index < hn_len {
                odd_hn.push(hn[odd_index]);
            } else {
                odd_hn.push(0);
            }
        }

        even_fft = dit_ffti(&even_hn);
        odd_fft = dit_ffti(&odd_hn);

        for r in 0..half_fetched_len {
            let factor = Factor::new(r as i32, fetched_len as i32);

            factors.push(factor);
            fft_results.push(even_fft[r] + factor * odd_fft[r]);
        }

        for r in 0..half_fetched_len {
            fft_results.push(even_fft[r] - factors[r] * odd_fft[r]);
        }
    } else {
        fft_results.push(Complex::new((hn[0] + hn[1]) as f64, 0.0));
        fft_results.push(Complex::new((hn[0] - hn[1]) as f64, 0.0));
    }

    fft_results
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

