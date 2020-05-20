use std::{ops, fmt};


// PartialEq -> comparision trait
// Debug -> to format a value using the {:?} formatter 

#[derive(PartialEq, Debug)]


// matrix, define a 'Matrix' type, overload the '+', '-', & '*' operators, & implement the 
// 'Display' trait on it by providing the following API:


// generic struct 
pub struct Matrix<T> {
	/// stores elements in [row-major order] 
	data: Vec<T>,
	/// number of rows
	row:usize,
	/// number of columns 
	col:usize,
}
// into_iter()
impl<T: Copy> Matrix<T> {
	/// Creates a new matrix of 'row' rows & 'col' columns, and initializes
	/// the matrix with the elements in 'values' in row-major order.
	pub fn new(row: usize, col: usize, values: &[T]) -> Matrix<T> {
		let mut v = Vec::new();
		for i in values.into_iter() {
			v.push(*i);
		}
		Matrix {
			data: v,
			row: row,
			col: col,
		}
	}
// empty vector 
	///Creates a new, empty matrix of 'row' rows & 'col' columns. 
	/// 'data' contains no element.
	pub fn new_empty( row: usize, col: usize) -> Matrix<T> {
		let mut v = Vec::new();
		Matrix {
			data: v,
			row: row,
			col: col,
		}
	}

	///Returns a shared reference to 'data'
	pub fn data(&self) -> &Vec<T> {
		// immutable reference 
		&self.data
	}

	/// Returns a mutable reference to 'data'
	pub fn mut_data(&mut self) -> &mut Vec<T> {
		// mutable reference 
		&mut self.data
	}

	/// Returns the number of rows & columns in the first & second
	/// elements of the tuple, respectively. 
	pub fn size(&self) -> (usize, usize) {
		(self.row, self.col)
	}
}

impl <T: ops::Add<Output = T> + Copy> ops::Add for Matrix<T> {
	type Output = Self;

	/// Returns the sum of self and rhs. If 'self.row != rhs.row || sef.col != rhs.col', panic.
	fn add(self, rhs: Self) -> Self::Output {
		let mut v = Vec::new();
		let length = self.row * self.col;
		for i in 0..length {
			v.push(self.data[i] + rhs.data[i]);
		}
		if self.row != rhs.row || self.col != rhs.col {
			panic!();
		}
		Matrix {
			data: v,
			row: self.row,
			col: self.col,
		}
	}
}

impl<T: ops::Sub<Output = T> + Copy> ops::Sub for Matrix<T> {
	type Output = Self;

	/// Returns the subtraction of rhs from self. If self.row != rhs.row || self.col != rhs.col, panic.
	fn sub(self, rhs: Self) -> Self::Output {
		let mut v = Vec::new();
		let length = self.row * self.col;
		for i in 0..length {
			v.push(self.data[i] - rhs.data[i]);
		}
		if self.row != rhs.row || self.col != rhs.col { 
			panic!();
		}
		Matrix {
			data: v,
			row: self.row,
			col: self.col,
		}
	}
}

impl<T: ops::Add<Output = T> + ops::Mul<Output = T> + Copy + Default> ops::Mul for Matrix<T> {
	type Output = Self;

	/// Returns the multiplication of self by rhs. If self.col != rhs.row, panic.
	fn mul(self, rhs:Self) -> Self::Output {
		// the number of col of self must be equal to the number of rows of rhs 
		if self.col != rhs.row {
			panic!();
		}
		// initialized vector u & v to hold self and rhs data 
		let mut u = Vec::new();
		let mut v = Vec::new();
		// initialized vector to hold the output vector 
		let mut w = Vec::new();
		let length = self.row * self.col;
		let length1 = rhs.row * rhs.col; 

		// iterate through to push data from self to v
		for g in 0..length {
			v.push(self.data[g]);
		}
		// iterate through to push data from rhs to u
		for h in 0..length {
			u.push(rhs.data[h]);
		}

		let nScol = self.col;
		let nRcol = rhs.col;
		for i in 0..self.row {
			// self.col needs to be equal to rhs.row
			for j in 0..rhs.col {
				let mut temp: T = Default::default();
				for k in 0..self.col {
					let x = v[i*nScol+k] * u[k*nRcol+j];
					temp = temp + x; // increment 
				}
				w.push(temp); // pushes value into w (vector that holds data for new matrix)
			}
		}


		Matrix {
			data: w, 
			row: self.row,
			col: rhs.col,
		}
	}
}

impl<T: fmt::Display> fmt::Display for Matrix<T> {
	/// Formats the matrix as follows:
	/// *Writes each row on a seperate line. No empty lines before or after any row. 
	/// * On each row, writes each element followed by a single space, except no space following the last element of the row.
	/// Outputs using 'write!(f,...)'.
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut v = Vec::new();
		let nCol = self.col;
		let length = self.row * self.col;
		for a in 0..length {
			v.push(&self.data[a]);
		}
		let mut s = String::new();

		for i in 0..self.row {
			for j in 0..self.col {
				let z = (v[i*nCol + j]).to_string();
				s.push_str(&z); // considers it to be a string
				s.push(' '); // push character is chill
			}
			// since after the last element of the row has no space
			// need to pop out the space 
			s.pop();
			s.push('\n');
		}
		write!(f, "{}", s)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	// use super::Matrix;
	#[test]
	fn test() {
		let x = Matrix::new(2,3, &[-2, -1, 0, 1,2,3]);
		let y = Matrix::new(2,3, &[0,0,0,0,0,0]);
		let z = Matrix::new(2,3, &[-2, -1, 0, 1,2,3]);
		assert_eq!(x + y, z);
		assert_eq!(format!("{}", z), "-2 -1 0\n1 2 3\n");
	}
}