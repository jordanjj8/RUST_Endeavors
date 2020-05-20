use std::{ops, fmt};

// PartialEq -> comparision trait
// Debug -> to format a value using the {:?} formatter 
#[derive(PartialEq, Debug)]
// generic struct 
pub struct Complex<T> {
	re: T,
	im: T,
}

// + operator overloaded with the Add trait 
impl < T:ops::Add<Output=T> > ops::Add for Complex<T> {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Complex { 
			re: self.re + rhs.re,
			im: self.im + rhs.im,
		}
	}
}

// - operator overloaded with the Sub trait
impl <T: ops::Sub<Output = T>> ops::Sub for Complex<T> {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		Complex {
			re: self.re - rhs.re,
			im: self.im - rhs.im,
		}
	}
}

impl< T: ops::Add<Output=T> + ops::Sub<Output=T> + ops::Mul<Output=T> + Copy> ops::Mul for Complex<T> {
	type Output = Self;

	fn mul(self, rhs:Self) -> Self::Output {
		Complex { 
			re: self.re * rhs.re - self.im * rhs.im,
			im: self.re * rhs.im + self.im * rhs.re,
		}
	}
}

impl<T: fmt::Display> fmt::Display for Complex<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}{:+}j", self.re, self.im)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn test0() {
    	let x = Complex{ re: 1, im: 2} + 
    			Complex{ re: -2, im: -3} * Complex{ re: 2, im: 3};
    	assert_eq!(x, Complex{ re: 6, im: -10});
    }

    #[test]
    fn test1() {
    	let x = Complex{ re: 1.0, im: 2.0} +
    			Complex{ re: -2.0, im: -3.0 } * Complex{ re:2.0, im: 3.0};
    	assert_eq!(x, Complex{ re: 6.0, im: -10.0});		
    }

    #[test] 
    fn test2() { 
    	assert_eq!(format!("{}", Complex{ re: 1, im: -2 }), "1-2j");
    	assert_eq!(format!("{}", Complex{ re: -1, im: 2}), "-1+2j");
    }
}
