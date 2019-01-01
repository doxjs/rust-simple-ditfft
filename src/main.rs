use std::i32;
use std::f64::consts::{ PI };

mod lib;

use crate::lib::{ dit_fftf, ComplexVec };

fn main() {
	let hn = xn(8);
	let fft = dit_fftf(&hn);

	display(fft);
}

fn display(v: ComplexVec) {
	for item in v.into_iter() {
		if item.im < 0_f64 {
			println!("{} - {}i", item.re, item.im.abs());
		} else {
			println!("{} + {}i", item.re, item.im.abs());
		}
	}
}

fn xn(n: i32) -> Vec<f64> {
	let mut v = vec![];

	for i in 0..n {
		v.push(i as f64);
	}

	v
}