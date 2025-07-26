use lib_a::add_one;

#[test]
fn integration_test_add_one() {
    assert_eq!(5, add_one(4));
}