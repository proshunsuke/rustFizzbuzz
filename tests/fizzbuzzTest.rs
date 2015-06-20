extern crate rustFizzbuzz;

use rustFizzbuzz::fizzbuzz;

#[test]
fn can_test() {
	assert_eq!(1, 1);
}

#[test]
fn _1is1() {
	assert_eq!("1", fb::fizzbuzz(1));
}
