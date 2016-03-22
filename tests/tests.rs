extern crate rust_rt;

#[test]
fn magnitude__valid_input__should_return_correct_value() {
    let v = vec![-3f64, 2f64, 5f64];
    let expectedMagnitude = (38 as f64).sqrt();
    assert_eq!(rust_rt::magnitude(&v), expectedMagnitude);
}

#[test]
fn normalize__valid_input__should_return_correct_value() {
    let v = vec![2f64, -5f64, 4f64];
    let expected_x = (2f64 / (3f64 * (5 as f64).sqrt()));
    let expected_y = -(5 as f64).sqrt() / 3f64;
    let expected_z = (4f64 / (3f64 *(5 as f64).sqrt()));

    let result = rust_rt::normalize(v);
    assert_eq!(result[0], expected_x);
    assert_eq!(result[1], expected_y);
    assert_eq!(result[2], expected_z);
}
