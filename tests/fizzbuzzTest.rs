extern crate rustFizzbuzz;

use rustFizzbuzz::fizzbuzz;

#[test]
fn can_test() {
	assert_eq!(1, 1);
}

#[test]
fn _1is1() {
	assert_eq!("1", fizzbuzz::fizzbuzz(1));
}

#[test]
fn _2is2() {
	assert_eq!("2", fizzbuzz::fizzbuzz(2));
}

#[test]
fn _3isfizz() {
	assert_eq!("fizz", fizzbuzz::fizzbuzz(3));
}

#[test]
fn _5isbuzz() {
	assert_eq!("buzz", fizzbuzz::fizzbuzz(5));
}

#[test]
fn _15isfizzbuzz() {
	assert_eq!("fizzbuzz", fizzbuzz::fizzbuzz(15));
}

