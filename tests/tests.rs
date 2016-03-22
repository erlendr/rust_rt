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

    let result = rust_rt::normalize(&v);
    assert_eq!(result[0], expected_x);
    assert_eq!(result[1], expected_y);
    assert_eq!(result[2], expected_z);
}

#[test]
fn negative__valid_input__should_return_correct_value() {
    let v = vec![2f64, 4f64, -5f64];

    let result = rust_rt::negative(&v);
    assert_eq!(-v[0], result[0]);
    assert_eq!(-v[1], result[1]);
    assert_eq!(-v[2], result[2]);
}

#[test]
fn dot_product__valid_input__should_return_correct_value() {
    let v1 = vec![1f64, 2f64, 3f64];
    let v2 = vec![3f64, 4f64, 5f64];

    let expected_result = 26f64;

    let result = rust_rt::dot_product(&v1, &v2);

    assert_eq!(expected_result, result);
}
