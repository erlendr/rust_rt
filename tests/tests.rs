use rust_rt::maths::vector3d;

#[test]
fn magnitude__valid_input__should_return_correct_value() {
    let v = vector3d::Vector3D { x: -3.0, y: 2.0, z: 5.0 };
    let expectedMagnitude = (38 as f64).sqrt();
    assert_eq!(v.magnitude(), expectedMagnitude);
}

#[test]
fn normalize__valid_input__should_return_correct_value() {
    let v = vector3d::Vector3D { x: 2.0, y: -5.0, z: 4.0 };

    let expected_x = 2.0 / (3.0 * (5 as f64).sqrt());
    let expected_y = -(5 as f64).sqrt() / 3.0;
    let expected_z = 4.0 / (3.0 *(5 as f64).sqrt());

    let result = v.normalize();
    assert_eq!(result.x, expected_x);
    assert_eq!(result.y, expected_y);
    assert_eq!(result.z, expected_z);
}

#[test]
fn negative__valid_input__should_return_correct_value() {
    let v = vector3d::Vector3D { x: 2.0, y: 4.0, z: -5.0 };

    let result = -v;
    assert_eq!(-2.0, result.x);
    assert_eq!(-4.0, result.y);
    assert_eq!(5.0, result.z);
}

#[test]
fn dot_product__valid_input__should_return_correct_value() {
    let v1 = vector3d::Vector3D { x: 1.0, y: 2.0, z: 3.0 };
    let v2 = vector3d::Vector3D { x: 3.0, y: 4.0, z: 5.0 };

    let expected_result = 26.0;

    let result = v1 * v2;

    assert_eq!(expected_result, result);
}

#[test]
fn cross_product__valid_input__should_return_correct_value() {
    let v1 = vector3d::Vector3D { x: 1.0, y: 2.0, z: 3.0 };
    let v2 = vector3d::Vector3D { x: 3.0, y: 4.0, z: 5.0 };

    let result = v1.cross_product(&v2);

    assert_eq!(-2.0, result.x);
    assert_eq!(4.0, result.y);
    assert_eq!(-2.0, result.z);
}

#[test]
fn add__valid_input__should_return_correct_result() {
    let v1 = vector3d::Vector3D { x: 1.0, y: 2.0, z: 3.0 };
    let v2 = vector3d::Vector3D { x: 3.0, y: 4.0, z: 5.0 };

    let result = v1 + v2;

    assert_eq!(4.0, result.x);
    assert_eq!(6.0, result.y);
    assert_eq!(8.0, result.z);
}

#[test]
fn mult__valid_input__should_return_correct_result() {
    let v = vector3d::Vector3D { x: 1.0, y: 2.0, z: 3.0};

    let result = v * 3.0;

    assert_eq!(3.0, result.x);
    assert_eq!(6.0, result.y);
    assert_eq!(9.0, result.z);
}
