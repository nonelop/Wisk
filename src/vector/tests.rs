use super::Vector3;

#[test]
fn test_vector3_neg() {
    let result = -Vector3::RIGHT;
    assert_eq!(result, Vector3::LEFT)
}

#[test]
fn test_vector3_add() {
    let result = Vector3::ZERO + Vector3::RIGHT;
    assert_eq!(result, Vector3::RIGHT)
}

#[test]
fn test_vector3_add_assign() {
    let mut left = Vector3::ZERO;
    left += Vector3::RIGHT;
    assert_eq!(left, Vector3::RIGHT)
}

#[test]
fn test_vector3_sub() {
    let result = Vector3::ZERO - Vector3::RIGHT;
    assert_eq!(result, Vector3::LEFT)
}

#[test]
fn test_vector3_sub_assign() {
    let mut left = Vector3::ZERO;
    left -= Vector3::RIGHT;
    assert_eq!(left, Vector3::LEFT)
}

#[test]
fn test_vector3_mul() {
    let result = Vector3::ZERO * Vector3::RIGHT;
    assert_eq!(result, Vector3::ZERO)
}

#[test]
fn test_vector3_mul_assign() {
    let mut left = Vector3::ZERO;
    left *= Vector3::RIGHT;
    assert_eq!(left, Vector3::ZERO)
}

#[test]
fn test_vector3_mul_f64() {
    let left = Vector3::ONE;
    let right: f64 = 5.6;

    assert_eq!(left * right, Vector3 { x: 5.6, y: 5.6, z: 5.6 });
    assert_eq!(right * left, Vector3 { x: 5.6, y: 5.6, z: 5.6 })
}

#[test]
fn test_vector3_div() {
    let result = Vector3::ZERO / Vector3::ONE;
    assert_eq!(result, Vector3::ZERO)
}

#[test]
fn test_vector3_div_assign() {
    let mut left = Vector3::ZERO;
    left /= Vector3::ONE;
    assert_eq!(left, Vector3::ZERO)
}

#[test]
fn test_vector3_div_f64() {
    let left = Vector3::ONE;
    let right: f64 = 2.5;

    assert_eq!(left / right, Vector3 { x: 0.4, y: 0.4, z: 0.4 })
}