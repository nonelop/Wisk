use super::Quaternion;

#[test]
fn test_quaternion_neg() {
    let result = -Quaternion::IDENTITY;
    assert_eq!(result, Quaternion { w: -1.0, x: 0.0, y: 0.0, z: 0.0 })
}

#[test]
fn test_quaternion_add() {
    let result = Quaternion::ZERO + Quaternion::IDENTITY;
    assert_eq!(result, Quaternion::IDENTITY)
}

#[test]
fn test_quaternion_add_assign() {
    let mut left = Quaternion::ZERO;
    left += Quaternion::IDENTITY;
    assert_eq!(left, Quaternion::IDENTITY)
}

#[test]
fn test_quaternion_sub() {
    let result = Quaternion::ZERO - Quaternion::IDENTITY;
    assert_eq!(result, -Quaternion::IDENTITY)
}

#[test]
fn test_quaternion_sub_assign() {
    let mut left = Quaternion::ZERO;
    left -= Quaternion::IDENTITY;
    assert_eq!(left, -Quaternion::IDENTITY)
}

#[test]
fn test_quaternion_mul() {
    let result = Quaternion::IDENTITY * Quaternion { w: 3.0, x: 1.0, y: 1.5, z: 0.0 };
    assert_eq!(result, Quaternion { w: 3.0, x: 1.0, y: 1.5, z: 0.0 })
}

#[test]
fn test_quaternion_mul_assign() {
    let mut left = Quaternion::IDENTITY;
    left *= Quaternion { w: 3.0, x: 1.0, y: 1.5, z: 0.0 };
    assert_eq!(left, Quaternion { w: 3.0, x: 1.0, y: 1.5, z: 0.0 })
}