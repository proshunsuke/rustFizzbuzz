pub fn fizzbuzz(n: i32) -> String {
	if n % 3 == 0 {
		"fizz".to_string()
	} else {
		n.to_string()
	}
}