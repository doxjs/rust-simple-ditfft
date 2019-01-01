use std::ops::{ Add, Sub, Mul, Div };
use std::f32::consts::{ PI };

#[derive(Debug, Copy, Clone)]
pub enum ComplexType {
	Polar,
	Euler,
}

#[derive(Debug, Copy, Clone)]
pub struct Complex {
	types: ComplexType,
	nums: (f32, f32),
}

impl Complex {
	pub fn from_f32(types: ComplexType, nums: (f32, f32)) -> Self {
		Complex {
			types,
			nums,
		}
	}

	pub fn from_i32(types: ComplexType, nums: (i32, i32)) -> Self {
		Complex {
			types,
			nums: (nums.0 as f32, nums.1 as f32)
		}
	}
}

impl Add for Complex {

	type Output = Complex;

	fn add(self, rhs: Complex) -> Self {
		if let ComplexType::Polar = self.types {
			if let ComplexType::Polar = rhs.types {
				let (rhs_real, rhs_imaginary) = rhs.nums;
				let (self_real, self_imaginary) = self.nums;

				Complex {
					types: ComplexType::Polar,
					nums: (rhs_real + self_real, rhs_imaginary + self_imaginary),
				}
			} else {
				let (amplitude, phase) = rhs.nums;
				let rhs = Complex::from_f32(ComplexType::Polar, 
					(amplitude * phase.cos(), amplitude * phase.sin()));
				println!("{:?}", rhs);
				self + rhs
			}
		} else {
			if let ComplexType::Euler = rhs.types {
				let (self_amplitude, self_phase) = self.nums;
				let (rhs_amplitude, rhs_phase) = rhs.nums;
				let lhs = Complex::from_f32(ComplexType::Polar, (self_amplitude * self_phase.cos(), self_amplitude * self_phase.sin()));
				let rhs = Complex::from_f32(ComplexType::Polar, (rhs_amplitude * rhs_phase.cos(), rhs_amplitude * rhs_phase.sin()));

				lhs + rhs
			} else {
				let (self_amplitude, self_phase) = self.nums;
				let lhs = Complex::from_f32(ComplexType::Polar, (self_amplitude * self_phase.cos(), self_amplitude * self_phase.sin()));

				lhs + rhs
			}
		}
	}
}

impl Sub for Complex {
	type Output = Complex;

	fn sub(self, rhs: Complex) -> Self {
		if let ComplexType::Polar = rhs.types {
			let (rhs_real, rhs_imaginary) = rhs.nums;

			self + Complex::from_f32(ComplexType::Polar, (-rhs_real, -rhs_imaginary))
		} else {
			let (rhs_amplitude, rhs_phase) = self.nums;

			self + Complex::from_f32(ComplexType::Euler, (-rhs_amplitude, rhs_phase))
		}
	}
}

// #[derive(Debug, Copy, Clone)]
// pub struct Complex(f32, f32);

// impl Complex {
// 	pub fn from_f32(real: f32, imaginary: f32) -> Self {
// 		Complex(real, imaginary)
// 	}
// }

// impl Add for Complex {
// 	type Output = Complex;
	
// 	fn add(self, rhs: Complex) -> Self {
// 		Complex(self.0 + rhs.0, self.1 + rhs.1)
// 	}
	
// }

// impl Sub for Complex {
// 	type Output = Complex;

// 	fn sub(self, rhs: Complex) -> Self {
// 		Complex(self.0 - rhs.0, self.1 - rhs.1)
// 	}
// }

// impl Mul for Complex {
// 	type Output = Complex;

// 	fn mul(self, rhs: Complex) -> Self {
// 		Complex(self.0 * rhs.0 - self.1 * rhs.1, self.0 * rhs.1 + self.1 * rhs.0)
// 	}
// }

// impl Div for Complex {
// 	type Output = Complex;

// 	fn div(self, rhs: Complex) -> Self {
// 		let conjugation = Complex(rhs.0, -rhs.1);
// 		let a = conjugation * self;
// 		let b = (conjugation * rhs).0;

// 		Complex(a.0 / b, a.1 / b)
// 	}
// }


// #[derive(Debug, Copy, Clone)]
// pub struct EulerFormula {
// 	amplitude: f32,
// 	phase: f32,
// }

// impl EulerFormula {
// 	pub fn from_f32(amplitude: f32, phase: f32) -> Self {
// 		EulerFormula {
// 			amplitude,
// 			phase,
// 		}
// 	}
// }
