#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operator {
	// '+' 
	Add,
	// '-'
	Sub,
	// '*'
	Mul,
	// '/'
	Div,
}


// if op1 on top is equal or greater, than pop!!
#[derive(Debug, PartialEq)]
pub enum InfixToken {
	Operator(Operator),
	Operand(isize),
	LeftParen,
	RightParen,
}

#[derive(Debug, PartialEq)]
pub enum PostfixToken {
	Operator(Operator),
	Operand(isize),
}

impl InfixToken {
	fn precedence(&self) -> i32 {
		match *self { 
			InfixToken::Operator(Operator::Add) | InfixToken::Operator(Operator::Sub) => 1,
			InfixToken::Operator(Operator::Mul) | InfixToken::Operator(Operator::Div) => 2,
			_ => 0
		}
	}
}

// Transforms an infix expression to a postfix expression.
//
// If the infix expression is valid, outputs 'Some(_)';
// Otherwise, outputs 'None'
pub fn infix_to_postfix(tokens: &[InfixToken]) -> Option<Vec<PostfixToken>> {
	let size = tokens.len();
	if size == 0 {
		return None;
	}
	// rule 2a  
	// check to see if expression begins with an operator or a RightParen
	match tokens[0] {
		InfixToken::RightParen => return None,
		InfixToken::Operator(Operator::Add) => return None,
		InfixToken::Operator(Operator::Sub) => return None,
		InfixToken::Operator(Operator::Mul) => return None,
		InfixToken::Operator(Operator::Div) => return None,
		_ => {}
	}
	//iterate through the infix input 
	//tokens is a slice  
	// i is a reference to the index of tokens, type &InfixToken
	for i in 0..size-1 {
		// checking rule 2b
		match (&tokens[i], &tokens[i +1]) {
			(&InfixToken::RightParen, &InfixToken::Operand(..)) => return None,
			(&InfixToken::RightParen, &InfixToken::LeftParen) => return None,
			(&InfixToken::LeftParen, &InfixToken::RightParen) => return None,
			(&InfixToken::Operator(..), &InfixToken::RightParen) => return None,
			(&InfixToken::Operator(..), &InfixToken::Operator(..)) => return None,
			(&InfixToken::Operand(..), &InfixToken::Operand(..)) => return None,
			(&InfixToken::Operand(..), &InfixToken::LeftParen) => return None,
			(&InfixToken::LeftParen, &InfixToken::Operator(..)) => return None,
			_ => {}
		}
	}
	//checking for the last token in the expression
	// cannot end with an operator or a left parenthesis 
	match tokens[size-1] {
		InfixToken::Operator(Operator::Add) => return None,
		InfixToken::Operator(Operator::Sub) => return None,
		InfixToken::Operator(Operator::Mul) => return None,
		InfixToken::Operator(Operator::Div) => return None,
		InfixToken::LeftParen => return None,
		_ => {}
	}
	

	// Transforming 

	// output vector to be outputed 
	let mut output: Vec<PostfixToken> = Vec::new();
	// create stack 
	// initializes vector s for stack
	let mut s: Vec<InfixToken> = Vec::new();
	let mut index = 0;
	// scans each token 
	for t in tokens {
		match *t {
			// if operand, then push to output 
			InfixToken::Operand(x) => output.push(PostfixToken::Operand(x)),
			// if left paren, then push into stack
			InfixToken::LeftParen => s.push(InfixToken::LeftParen),
			// if right paren, 1. pop 2. output operators till left paren is left
			// pop left paren and discard 
			InfixToken::RightParen => {
				//let mut k = s.len();
				if s.len() == 0 {
					return None;
				}
				while s[s.len()-1] != InfixToken::LeftParen {
					let x = s.pop().unwrap();
					match &x {
						&InfixToken::Operator(Operator::Add) => output.push(PostfixToken::Operator(Operator::Add)),
						&InfixToken::Operator(Operator::Sub) => output.push(PostfixToken::Operator(Operator::Sub)),
						&InfixToken::Operator(Operator::Mul) => output.push(PostfixToken::Operator(Operator::Mul)),
						&InfixToken::Operator(Operator::Div) => output.push(PostfixToken::Operator(Operator::Div)),
						_ => {}
					}
				}
				// if '(' on stack, discard '(' 
				if s[s.len()-1] == InfixToken::LeftParen {
					s.pop();
				}
			}
			// if operator...
			InfixToken::Operator(ref op) => {
				let mut Slen = s.len();
				// if the stack is empty, then push onto stack
				if Slen == 0 {
					s.push(InfixToken::Operator(*op));
				} else if s[Slen-1] != InfixToken::LeftParen {
					let mut op1 = InfixToken::Operator(*op);
					let mut op2 = InfixToken::Operator(Operator::Add);
					loop {
						// if the stack is empty, go ahead and push the current op1 to the stack
						if Slen == 0 {

							match &op1 {
								&InfixToken::Operator(Operator::Add) => s.push(InfixToken::Operator(Operator::Add)),
								&InfixToken::Operator(Operator::Sub) => s.push(InfixToken::Operator(Operator::Sub)),
								&InfixToken::Operator(Operator::Mul) => s.push(InfixToken::Operator(Operator::Mul)),
								&InfixToken::Operator(Operator::Div) => s.push(InfixToken::Operator(Operator::Div)),
								_ => {}
							}
							break;
						}
						match &s[Slen -1] {
							&InfixToken::Operator(Operator::Add) => op2 = InfixToken::Operator(Operator::Add),
							&InfixToken::Operator(Operator::Sub) => op2 = InfixToken::Operator(Operator::Sub),
							&InfixToken::Operator(Operator::Mul) => op2 = InfixToken::Operator(Operator::Mul),
							&InfixToken::Operator(Operator::Div) => op2 = InfixToken::Operator(Operator::Div),
							_ => {}

						}
						// if top stack precedence (op2) > = token precedence (op1) 
						// then pop & output operator from stack (op2)  
						if op2.precedence() >= op1.precedence() {
							let p = s.pop().unwrap();
							match &p {
								&InfixToken::Operator(Operator::Add) => output.push(PostfixToken::Operator(Operator::Add)),
								&InfixToken::Operator(Operator::Sub) => output.push(PostfixToken::Operator(Operator::Sub)),
								&InfixToken::Operator(Operator::Mul) => output.push(PostfixToken::Operator(Operator::Mul)),
								&InfixToken::Operator(Operator::Div) => output.push(PostfixToken::Operator(Operator::Div)),
								_ => {}
							}
							Slen = s.len();
						} else {
							match &op1 {
								&InfixToken::Operator(Operator::Add) => s.push(InfixToken::Operator(Operator::Add)),
								&InfixToken::Operator(Operator::Sub) => s.push(InfixToken::Operator(Operator::Sub)),
								&InfixToken::Operator(Operator::Mul) => s.push(InfixToken::Operator(Operator::Mul)),
								&InfixToken::Operator(Operator::Div) => s.push(InfixToken::Operator(Operator::Div)),
								_ => {}
							}
							break;
						}
 
					} // end of loop

				 // runs if the stack starts with a left paren
				} else {
					match &tokens[index] {
						&InfixToken::Operator(Operator::Add) => s.push(InfixToken::Operator(Operator::Add)),
						&InfixToken::Operator(Operator::Sub) => s.push(InfixToken::Operator(Operator::Sub)),
						&InfixToken::Operator(Operator::Mul) => s.push(InfixToken::Operator(Operator::Mul)),
						&InfixToken::Operator(Operator::Div) => s.push(InfixToken::Operator(Operator::Div)),
						_ => {}
					}
				}	
			}

		} // end of match statement
	// increment index	
	index = index + 1; 
	} // end of scan tokens 
	let mut sizeFs = s.len();

	// pop and output all the remaining tokens on the stack
	while sizeFs >= 1 {
		let remaining = s.pop().unwrap();
		match &remaining {
			&InfixToken::Operator(Operator::Add) => output.push(PostfixToken::Operator(Operator::Add)),
			&InfixToken::Operator(Operator::Sub) => output.push(PostfixToken::Operator(Operator::Sub)),
			&InfixToken::Operator(Operator::Mul) => output.push(PostfixToken::Operator(Operator::Mul)),
			&InfixToken::Operator(Operator::Div) => output.push(PostfixToken::Operator(Operator::Div)),
			&InfixToken::LeftParen => return None,
			_ => {}
		}
		sizeFs = s.len();
	}

	return Some(output);

}

#[cfg(test)]
mod tests {
	use super::Operator;
	use super::InfixToken;
	use super::PostfixToken;
	use super::infix_to_postfix;
    #[test]
    fn test1() {

    	let optr1 = InfixToken::Operator(Operator::Add);
    	let optr2 = InfixToken::Operator(Operator::Mul);
    	let v1 = InfixToken::Operand(5);
    	let v2 = InfixToken::Operand(2);
    	let v3 = InfixToken::Operand(3);
    	// tokens 
    	let tokens:[InfixToken;5] = [v1, optr1, v2, optr2, v3]; 
    	let mut p: Vec<PostfixToken> = Vec::new();
    	let optr11 = PostfixToken::Operator(Operator::Add);
    	let optr22 = PostfixToken::Operator(Operator::Mul);
    	let v11 = PostfixToken::Operand(5);
    	let v22 = PostfixToken::Operand(2);
    	let v33 = PostfixToken::Operand(3);
    	p.push(v11);
    	p.push(v22);
    	p.push(v33);
    	p.push(optr22);
    	p.push(optr11);
    	// comparing to see if valid 
    	println!("{:?}", p);
    	println!("{:?}", infix_to_postfix(&tokens));
    	assert_eq!(Some(p),infix_to_postfix(&tokens));
    }
    #[test]
    fn test2() {

    	let optr1 = InfixToken::Operator(Operator::Add);
    	let optr2 = InfixToken::Operator(Operator::Add);
    	let v1 = InfixToken::Operand(5);
    	let v2 = InfixToken::Operand(2);
    	let v3 = InfixToken::Operand(3);
    	// tokens 
    	let tokens:[InfixToken;5] = [v1, optr1, v2, optr2, v3]; 
    	let mut p: Vec<PostfixToken> = Vec::new();
    	let optr11 = PostfixToken::Operator(Operator::Add);
    	let optr22 = PostfixToken::Operator(Operator::Add);
    	let v11 = PostfixToken::Operand(5);
    	let v22 = PostfixToken::Operand(2);
    	let v33 = PostfixToken::Operand(3);
    	p.push(v11);
    	p.push(v22);
    	p.push(optr22);
    	p.push(v33);
    	p.push(optr11);
    	// comparing to see if valid 
    	println!("{:?}", p);
    	println!("{:?}", infix_to_postfix(&tokens));
    	assert_eq!(Some(p),infix_to_postfix(&tokens));
    }

    #[test]
    fn testpar1() {
    	let optr1 = InfixToken::Operator(Operator::Mul);
    	let optr2 = InfixToken::Operator(Operator::Add);
    	let pare1 = InfixToken::LeftParen;
    	let pare2 = InfixToken::RightParen;
    	let v1 = InfixToken::Operand(5);
    	let v2 = InfixToken::Operand(2);
    	let v3 = InfixToken::Operand(3);
    	// tokens 
    	let tokens:[InfixToken;7] = [pare1, v1, optr1, v2, pare2, optr2, v3]; 
    	let mut p: Vec<PostfixToken> = Vec::new();
    	let optr11 = PostfixToken::Operator(Operator::Add);
    	let optr22 = PostfixToken::Operator(Operator::Mul);
    	let v11 = PostfixToken::Operand(5);
    	let v22 = PostfixToken::Operand(2);
    	let v33 = PostfixToken::Operand(3);
    	p.push(v11);
    	p.push(v22);
    	p.push(optr22);
    	p.push(v33);
    	p.push(optr11);
    	// comparing to see if valid 
    	println!("{:?}", p);
    	println!("{:?}", infix_to_postfix(&tokens));
    	assert_eq!(Some(p),infix_to_postfix(&tokens));
    }

    #[test] 
    fn testwpar1() {
    	let optr1 = InfixToken::Operator(Operator::Mul);
    	let optr2 = InfixToken::Operator(Operator::Add);
    	let pare1 = InfixToken::LeftParen;
    	let pare11 = InfixToken::LeftParen;
    	let pare2 = InfixToken::RightParen;    	
    	let pare3 = InfixToken::RightParen;
    	let v1 = InfixToken::Operand(5);
    	let v2 = InfixToken::Operand(2);
    	let v3 = InfixToken::Operand(3);
    	// tokens 
    	let tokens:[InfixToken;9] = [pare1, pare11, v1, optr1, v2,pare3, pare2, optr2, v3]; 
    	let mut p: Vec<PostfixToken> = Vec::new();
    	let optr11 = PostfixToken::Operator(Operator::Add);
    	let optr22 = PostfixToken::Operator(Operator::Mul);
    	let v11 = PostfixToken::Operand(5);
    	let v22 = PostfixToken::Operand(2);
    	let v33 = PostfixToken::Operand(3);
    	p.push(v11);
    	p.push(v22);
    	p.push(optr22);
    	p.push(v33);
    	p.push(optr11);
    	// comparing to see if valid 
    	println!("{:?}", p);
    	println!("{:?}", infix_to_postfix(&tokens));
    	assert_eq!(Some(p),infix_to_postfix(&tokens));
    }
    #[test]
    fn testcase1 () {
    //INFIX: 3 * 4 + 6 + 4 * 2
    //POSTFIX: 3 4 * 6 + 4 2 * +
        let x = &[InfixToken::Operand(3),
                  InfixToken::Operator(Operator::Mul),
                  InfixToken::Operand(4),
                  InfixToken::Operator(Operator::Add),
                  InfixToken::Operand(6),
                  InfixToken::Operator(Operator::Add),
                  InfixToken::Operand(4),
                  InfixToken::Operator(Operator::Mul),
                  InfixToken::Operand(2),
              ];
        let y = Some(vec![
                 PostfixToken::Operand(3),
                 PostfixToken::Operand(4),
                 PostfixToken::Operator(Operator::Mul),
                 PostfixToken::Operand(6),
                 PostfixToken::Operator(Operator::Add),
                 PostfixToken::Operand(4),
                 PostfixToken::Operand(2),
                 PostfixToken::Operator(Operator::Mul),
                 PostfixToken::Operator(Operator::Add)]);
        assert_eq!(y, infix_to_postfix(x));
	}
	#[test]
	fn testcase2(){
    /* INFIX: 3 - 4 + 6 - 4 + 6 * 10
       POSTFIX: 3 4 - 6 + 4 - 6 10 * +
    */
let x = &[InfixToken::Operand(3),
          InfixToken::Operator(Operator::Sub),
          InfixToken::Operand(4),
          InfixToken::Operator(Operator::Add),
          InfixToken::Operand(6),
          InfixToken::Operator(Operator::Sub),
          InfixToken::Operand(4),
          InfixToken::Operator(Operator::Add),
          InfixToken::Operand(6),
          InfixToken::Operator(Operator::Mul),
          InfixToken::Operand(10),
      ];
      let y = Some(vec![
         PostfixToken::Operand(3),
         PostfixToken::Operand(4),
         PostfixToken::Operator(Operator::Sub),
         PostfixToken::Operand(6),
         PostfixToken::Operator(Operator::Add),
         PostfixToken::Operand(4),
         PostfixToken::Operator(Operator::Sub),
         PostfixToken::Operand(6),
         PostfixToken::Operand(10),
         PostfixToken::Operator(Operator::Mul),
         PostfixToken::Operator(Operator::Add)]);
      assert_eq!(y, infix_to_postfix(x));
}
#[test]
fn testcase3(){
    // INFIX: 6 + 4 + 6 / 10 * 5 * 4
    // POSTFIX: 6 4 + 6 10 / 5 * 4 * +
          let x = &[
              InfixToken::Operand(6),
              InfixToken::Operator(Operator::Add),
              InfixToken::Operand(4),
              InfixToken::Operator(Operator::Add),
              InfixToken::Operand(6),
              InfixToken::Operator(Operator::Div),
              InfixToken::Operand(10),
              InfixToken::Operator(Operator::Mul),
              InfixToken::Operand(5),
              InfixToken::Operator(Operator::Mul),
              InfixToken::Operand(4),
          ];
          let y = Some(vec![
             PostfixToken::Operand(6),
             PostfixToken::Operand(4),
             PostfixToken::Operator(Operator::Add),
             PostfixToken::Operand(6),
             PostfixToken::Operand(10),
             PostfixToken::Operator(Operator::Div),
             PostfixToken::Operand(5),
             PostfixToken::Operator(Operator::Mul),
             PostfixToken::Operand(4),
             PostfixToken::Operator(Operator::Mul),
             PostfixToken::Operator(Operator::Add)]);
          assert_eq!(y, infix_to_postfix(x));


}
#[test]
fn testcase4 (){
    // INFIX: 3 * (4 + 5)
    // POSTFIX: 3 4 5 + *
let x = &[InfixToken::Operand(3),
          InfixToken::Operator(Operator::Mul),
          InfixToken::LeftParen,
          InfixToken::Operand(4),
          InfixToken::Operator(Operator::Add),
          InfixToken::Operand(5),
          InfixToken::RightParen];
      let y = Some(vec![
         PostfixToken::Operand(3),
         PostfixToken::Operand(4),
         PostfixToken::Operand(5),
         PostfixToken::Operator(Operator::Add),
         PostfixToken::Operator(Operator::Mul)]);
      assert_eq!(y, infix_to_postfix(x));
}
#[test]
fn testcase5(){
    //INFIX: 3 + (4) + (4)
    //POSTFIX: 3 4 + 4 +
     let x = &[InfixToken::Operand(3),
          InfixToken::Operator(Operator::Add),
          InfixToken::LeftParen,
          InfixToken::Operand(4),
          InfixToken::RightParen,
          InfixToken::Operator(Operator::Add),
          InfixToken::LeftParen,
          InfixToken::Operand(4),
          InfixToken::RightParen,
      ];
      let y = Some(vec![
         PostfixToken::Operand(3),
         PostfixToken::Operand(4),
         PostfixToken::Operator(Operator::Add),
         PostfixToken::Operand(4),
         PostfixToken::Operator(Operator::Add)]);
      assert_eq!(y, infix_to_postfix(x));
}
#[test]
fn testcase6 (){
    //INFIX: (5+3)-6
    //POSTFIX: 5 3 + 6 -
          let x = &[
              InfixToken::LeftParen,
              InfixToken::Operand(5),
              InfixToken::Operator(Operator::Add),
              InfixToken::Operand(3),
              InfixToken::RightParen,
              InfixToken::Operator(Operator::Sub),
              InfixToken::Operand(6),
          ];
          let y = Some(vec![
             PostfixToken::Operand(5),
             PostfixToken::Operand(3),
             PostfixToken::Operator(Operator::Add),
             PostfixToken::Operand(6),
             PostfixToken::Operator(Operator::Sub)]);
          assert_eq!(y, infix_to_postfix(x));
}
#[test]
fn testcase7 (){
    //INFIX: 5(5+3)-6
    //POSTFIX: ERROR
    let x = &[
              InfixToken::Operand(5),
              InfixToken::LeftParen,
              InfixToken::Operand(5),
              InfixToken::Operator(Operator::Add),
              InfixToken::Operand(3),
              InfixToken::RightParen,
              InfixToken::Operator(Operator::Sub),
              InfixToken::Operand(6),
          ];
    let y = None;
    assert_eq!(y, infix_to_postfix(x));
}
#[test]
fn testcase8 (){
    //INFIX: 3 5 6
    //POSTFIX: ERROR
    let x = &[
          InfixToken::Operand(3),
          InfixToken::Operand(5),
          InfixToken::Operand(6),
    ];
    let y = None;
    assert_eq!(y,infix_to_postfix(x));
}
#[test]
fn testcase9 (){
    //INFX: (3 + 5))
    //POSTFIX: ERROR
    let x= &[
        InfixToken::LeftParen,
        InfixToken::Operand(3),
        InfixToken::Operator(Operator::Add),
        InfixToken::Operand(5),
        InfixToken::RightParen,
        InfixToken::RightParen,
    ];
    let y = None;
    assert_eq!(y,infix_to_postfix(x));
}
#[test]
fn testcase10 () {
    //INFIX: ((3+5))
    //POSTFIX: 3 5 +
    let x = &[
    InfixToken::LeftParen,
    InfixToken::LeftParen,
    InfixToken::Operand(3),
    InfixToken::Operator(Operator::Add),
    InfixToken::Operand(5),
    InfixToken::RightParen,
    InfixToken::RightParen,
    ];
    let y = Some(vec![
   PostfixToken::Operand(3),
   PostfixToken::Operand(5),
   PostfixToken::Operator(Operator::Add)]);
   assert_eq!(y, infix_to_postfix(x));
}
#[test]
fn testcase11 (){
    //INFIX: 3 + +
    //POSTFIX: ERROR
    let x = &[
    InfixToken::Operand(3),
    InfixToken::Operator(Operator::Add),
    InfixToken::Operator(Operator::Add)];

    let y = None;
    assert_eq!(y, infix_to_postfix(x));
}
#[test]
fn testcase12 (){
    //INFX: ((3 + 5)
    //POSTFIX: ERROR
    let x= &[
        InfixToken::LeftParen,
        InfixToken::LeftParen,
        InfixToken::Operand(3),
        InfixToken::Operator(Operator::Add),
        InfixToken::Operand(5),
        InfixToken::RightParen,
    ];
    let y = None;
    assert_eq!(y,infix_to_postfix(x));
}
#[test]
fn testcase13 (){
    //INFIX: (3+5)(3+5)
    //POSTFIX: ERROR
    let x= &[
        InfixToken::LeftParen,
        InfixToken::Operand(3),
        InfixToken::Operator(Operator::Add),
        InfixToken::Operand(5),
        InfixToken::RightParen,
        InfixToken::LeftParen,
        InfixToken::Operand(3),
        InfixToken::Operator(Operator::Add),
        InfixToken::Operand(5),
        InfixToken::RightParen,
    ];
    let y = None;
    assert_eq!(y, infix_to_postfix(x));
}
#[test]
fn testcase14 (){
    //INFIX: (3+5)5
    //POSTFIX: ERROR
    let x= &[
        InfixToken::LeftParen,
        InfixToken::Operand(3),
        InfixToken::Operator(Operator::Add),
        InfixToken::Operand(5),
        InfixToken::RightParen,
        InfixToken::Operand(5),
    ];
    let y = None;
    assert_eq!(y, infix_to_postfix(x));
}
#[test]
fn testcase15 (){
    //INFIX: ) 3 + 5 (
    //POSTFIX: Error
    let x= &[
        InfixToken::RightParen,
        InfixToken::Operand(3),
        InfixToken::Operator(Operator::Add),
        InfixToken::Operand(5),
        InfixToken::LeftParen,
    ];
    let y = None;
    assert_eq!(y, infix_to_postfix(x));
}
#[test]
fn testcase16 (){
    //INFIX: 3 4 +
    //POSTFIX: ERROR
    let x = &[
    InfixToken::Operand(3),
    InfixToken::Operand(4),
    InfixToken::Operator(Operator::Add)];

    let y = None;
    assert_eq!(y, infix_to_postfix(x));
}
#[test]
fn testcase17 (){
    let x = &[];
    let y = None;
    assert_eq!(y, infix_to_postfix(x));
}
#[test]
fn testcase18 (){
    //INFIX: +3
    //POSTFIX: ERROR
    let x = &[
    InfixToken::Operator(Operator::Add),
    InfixToken::Operand(3)];

    let y = None;
    assert_eq!(y, infix_to_postfix(x));
}
#[test]
fn testcase19 (){
    //INFIX: 2+*3
    //POSTFIX: ERROR
    let x = &[
    InfixToken::Operand(3),
    InfixToken::Operator(Operator::Add),
    InfixToken::Operator(Operator::Mul),
    InfixToken::Operand(3)];

    let y = None;
    assert_eq!(y, infix_to_postfix(x));
}
#[test]
fn testcase20 (){
    //INFIX: +(3+3)
    //POSTFIX: ERROR
    let x = &[
    InfixToken::Operator(Operator::Add),
    InfixToken::LeftParen,
    InfixToken::Operand(3),
    InfixToken::Operator(Operator::Add),
    InfixToken::Operand(3),
    InfixToken::RightParen];

    let y = None;
    assert_eq!(y, infix_to_postfix(x));
}
#[test]
fn testcase21 (){
    //INFIX: (3+3+)
    //POSTFIX: ERROR
    let x = &[
    InfixToken::LeftParen,
    InfixToken::Operand(3),
    InfixToken::Operator(Operator::Add),
    InfixToken::Operand(3),
    InfixToken::Operator(Operator::Add),
    InfixToken::RightParen];

    let y = None;
    assert_eq!(y, infix_to_postfix(x));

}
#[test]
fn testcase22 (){
    //INFIX: (3+3+)+3
    //POSTFIX: ERROR
    let x = &[
    InfixToken::LeftParen,
    InfixToken::Operand(3),
    InfixToken::Operator(Operator::Add),
    InfixToken::Operand(3),
    InfixToken::Operator(Operator::Add),
    InfixToken::RightParen,
    InfixToken::Operator(Operator::Add),
    InfixToken::Operand(3)];

    let y = None;
    assert_eq!(y, infix_to_postfix(x));

}
#[test]
fn testcase23 (){
    //INFIX: (3+4)+4(3-4)
    //POSTFIX: ERROR
    let x = &[
    InfixToken::LeftParen,
    InfixToken::Operand(3),
    InfixToken::Operator(Operator::Add),
    InfixToken::Operand(4),
    InfixToken::RightParen,
    InfixToken::Operator(Operator::Add),
    InfixToken::Operand(4),
    InfixToken::LeftParen,
    InfixToken::Operand(3),
    InfixToken::Operator(Operator::Sub),
    InfixToken::Operand(4),
    InfixToken::RightParen
    ];

    let y = None;
    assert_eq!(y, infix_to_postfix(x));
}
#[test]
fn testcase24 (){
    //INFIX: ()
    //POSTFIX: ERROR
    let x = &[
    InfixToken::LeftParen,
    InfixToken::RightParen
    ];

    let y = None;
    assert_eq!(y, infix_to_postfix(x));
}
#[test]
fn testcase25 () {
    //INFIX: )(3+4
    //POSTFIX: ERROR
    let x = &[
    InfixToken::RightParen,
    InfixToken::LeftParen,
    InfixToken::Operand(3),
    InfixToken::Operator(Operator::Add),
    InfixToken::Operand(4)];

    let y = None;
    assert_eq!(y, infix_to_postfix(x));
}
#[test]
fn testcase26 (){
    //INFIX: 3+4-
    //POSTFIX: ERROR
    let x = &[
    InfixToken::Operand(3),
    InfixToken::Operator(Operator::Add),
    InfixToken::Operand(4),
    InfixToken::Operator(Operator::Sub),];

    let y = None;
    assert_eq!(y, infix_to_postfix(x));
}
#[test]
fn testcase27 (){
    //INFIX: (3+4)
    //POSTFIX: 3 4 +
    let x = &[
    InfixToken::LeftParen,
    InfixToken::Operand(3),
    InfixToken::Operator(Operator::Add),
    InfixToken::Operand(4),
    InfixToken::RightParen];

    let y = None;
    assert_eq!(y, infix_to_postfix(x));
}
#[test]
fn testcase28 (){
    //INFIX: (3+4)() //Test case 13 & 19 on gradebot :D
    //POSTFIX: ERROR
    let x = &[
    InfixToken::LeftParen,
    InfixToken::Operand(3),
    InfixToken::Operator(Operator::Add),
    InfixToken::Operand(4),
    InfixToken::RightParen,
    InfixToken::LeftParen,
    InfixToken::RightParen];

    let y = None;
    assert_eq!(y, infix_to_postfix(x));
}
}
