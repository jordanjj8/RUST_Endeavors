use std::cmp::Ordering::*;

// rust cannot know how much memory to allocate to the data type 
// fill the field with a pointer

#[derive(PartialEq)]
pub struct Tree<T> {
	key: Option<T>,
	left: Option<Box<Tree<T>>>, // recursive call 
	right: Option<Box<Tree<T>>>,
}



impl <T: Ord > Tree<T> {
	/// Creates an empty Tree
	pub fn new() -> Self {
		// returns a struct Tree that has no root 
		Tree {
			key: None,
			left: None,
			right: None,
		}

	}

	/// Returns 'false' if 'key' already exists in the tree, & 'true' otherwise.
	pub fn insert(&mut self, key:T) -> bool {
		// self is the tree
		match self.key {
			// runs if there is no tree 
			None => {
				self.key = Some(key);
				return true;
			},
			// runs if there is a tree 
			Some(ref mut x) => { 
				// checks to see if the current key in tree is equal
				//if key == x {
				//	return false;
			//}

				// node is a mtable reference to either right or left side
				let node = if key == *x { return false;} else if key < *x { &mut self.left} else { &mut self.right};
				match node {
					// if there is no right, then insert
					&mut None => {
						// creates new node 
						let new = Tree { key: Some(key), left: None, right: None,};
						// boxes the node into a pointer and wraps up the pointer
						let boxed_new = Some(Box::new(new));
						// the right or left side now points to the newly added node 
						*node = boxed_new;
						return true;
					},

					// if there is a right, recursive call
					&mut Some(ref mut y) => y.insert(key),
				}
			// end of some(x)
			}, 
		// end of match statement 	
		}

	}

	/// Returns 'true' if 'key' exists in the tree, & 'false' otherwise.
	pub fn find(&self, key: &T) -> bool {
		// self is the tree
		match self.key {
			// runs if there is no tree 
			None => {
				// return false if there is none 
				return false;
			},
			// runs if there is a tree 
			Some(ref x) => { 
				if *key == *x {
					return true;
				} else if *key < *x {
					match self.left {
						None => return false,

						Some(ref y) => y.find(key), 
					}
				} else {
					match self.right {
						None => return false,

						Some(ref y) => y.find(key),
					}
				}
			// end of some(x)
			}, 
		// end of match statement 	
		}

	}

	// Returns the preorder traversal of the tree
	pub fn preorder(&self) -> Vec<&T> {
		let mut output: Vec<&T> = Vec::new(); 
		match self.key {
			None => {},
			Some(ref x) => {
				// need reference 
				output.push(&x);
				match self.left {
					None => {},
					Some(ref left) => output.append(&mut left.preorder()),
				}
				match self.right {
					None => {},
					Some(ref right) => output.append(&mut right.preorder()),
				}
			},
		}
		output
	}

	/// Returns the inorder traversal of the tree 
	pub fn inorder(&self) -> Vec<&T> {
		let mut output: Vec<&T> = Vec::new(); 
		match self.key {
			None => {},
			Some(ref x) => {
				match self.left {
					None => {},
					Some(ref left) => output.append(&mut left.inorder()),
				}
				// need reference 
				output.push(&x);

				match self.right {
					None => {},
					Some(ref right) => output.append(&mut right.inorder()),
				}
			},
		}
		output
	}

	/// Returns the postorder taversal of the tree. 
	pub fn postorder(&self) -> Vec<&T> {
		let mut output: Vec<&T> = Vec::new(); 
		match self.key {
			None => {},
			Some(ref x) => {
				match self.left {
					None => {},
					Some(ref left) => output.append(&mut left.postorder()),
				}
				match self.right {
					None => {},
					Some(ref right) => output.append(&mut right.postorder()),
				}
				// need reference 
				output.push(&x);
			},
		}
		output
	}


}

#[cfg(test)]
mod tests {
	use super::Tree;

    #[test]
    fn testcase1() {
    	let mut x: Tree<i32> = Tree::new();
    	x.insert(4);
    	x.insert(2);
    	x.insert(5);
    	x.insert(1);
    	x.insert(3);
    	x.insert(6);
    	x.insert(4);
    	assert_eq!(x, Tree { 
    		key: Some(4),
    		left: Some(Box::new(Tree {
    			key: Some(2),
    			left: Some(Box::new(Tree {key: Some(1), left: None, right: None,})),
    			right: Some(Box::new( Tree { key: Some(3), left: None, right: None })),
    		})),
    		right: Some(Box::new(Tree {
    			key: Some(5),
    			left: None,
    			right: Some(Box::new( Tree { key: Some(6), left: None, right: None })),
    		})),
    	});
    	// should be false
    	//x.insert(4);

    }
    #[test]
    fn testc2() {
    let mut x: Tree<i32> = Tree::new();
    	x.insert(4);
    x.insert(2);
    	x.insert(5);
    x.insert(1);
    	x.insert(3);
   	x.insert(6);
    	x.insert(4);
   	let y = x.find(&7);
    	assert_eq!(y, false);
    }
     #[test]
    fn testc4() {
    let mut x: Tree<i32> = Tree::new();
    	x.insert(4);
    x.insert(2);
    	x.insert(5);
    x.insert(1);
    	x.insert(3);
   	x.insert(6);
    	let y = x.insert(4);
    	assert_eq!(y, false);
    }
    #[test]
    fn testc3() {
    	let mut x: Tree<i32> = Tree::new();
    	x.insert(4);
    	x.insert(2);
    	x.insert(5);
    	x.insert(1);
    	x.insert(3);
    	x.insert(6);
    	let pre = x.preorder();
    	let a = 4; 
    	let b = 2;
    	let c= 1;
    	let d= 3;
    	let e=  5;
    	let f= 6;
   		let mut order: Vec<&i32> = vec![&a,&b,&c,&d,&e,&f];
    	assert_eq!(pre, order);
    } 
    #[test]
    fn testc9() {
    	let mut x: Tree<i32> = Tree::new();
    	x.insert(4);
    	x.insert(2);
    	x.insert(5);
    	x.insert(1);
    	x.insert(3);
    	x.insert(6);
    	let pre = x.inorder();
    	let a = 1; 
    	let b = 2;
    	let c= 3;
    	let d= 4;
    	let e=  5;
    	let f= 6;
   		let mut order: Vec<&i32> = vec![&a,&b,&c,&d,&e,&f];
    	assert_eq!(pre, order);
    } 
    #[test]
    fn testc10() {
    	let mut x: Tree<i32> = Tree::new();
    	x.insert(4);
    	x.insert(2);
    	x.insert(5);
    	x.insert(1);
    	x.insert(3);
    	x.insert(6);
    	let pre = x.postorder();
    	let a = 1; 
    	let b = 3;
    	let c= 2;
    	let d= 6;
    	let e=  5;
    	let f= 4;
   		let mut order: Vec<&i32> = vec![&a,&b,&c,&d,&e,&f];
    	assert_eq!(pre, order);
    } 



}
