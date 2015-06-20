pub fn fizzbuzz(n: i32) -> String {
	if n % 3 == 0 && n % 5 == 0 {
		"fizzbuzz".to_string()
	} else if n % 3 == 0 {
		"fizz".to_string()
	} else if n % 5 == 0 {
		"buzz".to_string()
	} else {
		n.to_string()
	}
}