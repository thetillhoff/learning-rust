use lib_b::{add_two, add, sub};

#[test]
fn integration_test_add_one() {
    assert_eq!(5, add_two(3));
}

#[test]
fn integration_test_adder() {
    assert_eq!(10, add::add(5, 5));
}

#[test]
fn integration_test_subtracter() {
    assert_eq!(0, sub::subtract(5, 5));
}