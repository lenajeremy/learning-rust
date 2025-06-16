use adder::{add, new_module};

#[test]
fn it_adds_two() {
    let result = add(2, 2);
    assert_eq!(result, 4, "Expected 2 + 2 to equal 4");
    let abs_result = new_module::abs(-5);
    assert_eq!(abs_result, 5, "Expected abs(-5) to equal 5");
}

