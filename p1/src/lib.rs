pub enum Operator {
	// '+'
	Add,
	// '-'
	Sub,
	// '*'
	Mul,
}
// access a slice like: tokens[0]
pub enum Token {
	Operator(Operator),
	Operand(isize),
}



/// Evaluates the postix expression
///
/// Input: a postfix expression, where each element contains an operator or operand. 
/// Returns: if postfix is valid: returns 'Some(value)';
///		otherwise, returns 'None'.
pub fn eval(tokens: &[Token]) -> Option<isize> {
	// intialize a vector v for the stack 
	let mut v = Vec::new();
	// interate through the stack 
	// tokens is a reference to Token
	for i in tokens {
		match *i {
			// if the value is of isize type 
			// v.push(x) is option type, 
			// if it has something, option::some(value) 
			Token::Operand(x) => v.push(x),
			// if the value is a '+'
			Token::Operator(Operator::Add) => {
				// panics if the stack has less than 2 values 
				// use 1 pattern for all cases of operators and then inside if then, check length then match again
				if v.len() < 2 { 
					return None;
				} else {
					let y = v.pop().unwrap(); 
					let z = v.pop().unwrap();
					v.push(y+z); 
				}
			}
			// if the value is a '-'
			Token::Operator(Operator::Sub) => {
				// panics if the stack has less than 2 values 
				if v.len() < 2 {
					return None;
				} else {
					let y = v.pop().unwrap();
					let z = v.pop().unwrap();
					v.push(z-y);
				}
			}
			// if the value is a '*'
			Token::Operator(Operator::Mul) => {
				// panics if the stack has less than 2 values 
				if v.len() < 2 {
					return None;
				} else {
					let y = v.pop().unwrap();
					let z = v.pop().unwrap();
					v.push(y*z);
				}
			}
		}
	}
	// if there is only 1 value on the stack, return the value 
	if v.len() == 1 {
		return Some(v.pop().unwrap());
	// if there is more than 1 value on the stack, return nothing
	} else { 
		return None;
	} 
}


#[cfg(test)]
mod tests {
	use super::Operator;
	use super::Token;
	use super::eval;
    #[test]
    fn it_works() {

    	let optr1 = Token::Operator(Operator::Add);
    	let optr2 = Token::Operator(Operator::Mul);
    	let v1 = Token::Operand(-1);
    	let v2 = Token::Operand(2);
    	let v3 = Token::Operand(3);
    	// tokens 
    	let tokens:[Token;5] = [v1, v2, v3, optr1, optr2 ]; 

    	// calling eval to eval the 
    	assert_eq!(Some(-5),eval(&tokens));
    }

    fn neg_works() {

    }
}