extern crate rust_s_exp;

use rust_s_exp::is_number;

#[test]
fn it_works_num() {
	// 合法数字
	assert!(is_number("123"));
	assert!(is_number("+12"));
	assert!(is_number("-89"));
	assert!(is_number("-.123"));
	assert!(is_number("098.765"));
	assert!(is_number(".8"));
	assert!(is_number("0."));
	assert!(is_number("+0"));
	assert!(is_number("+7."));
	assert!(is_number("6."));
	assert!(is_number("-345.876"));

	// 非法数字
	assert!(!is_number("+."));
	assert!(!is_number("."));
	assert!(!is_number("-"));
	assert!(!is_number("0a9"));
	assert!(!is_number("abcde"));
	assert!(!is_number("+abc"));
	assert!(!is_number("-+"));
	assert!(!is_number("-0-1-"));
}