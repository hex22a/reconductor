use newtype_derive::Newtype;

#[derive(Newtype, Debug, PartialEq)]
struct Meters(f64);

#[derive(Newtype)]
struct Name(String);

#[test]
fn test_into_inner() {
    // Arrange
    let expected_meters = 1.5;

    // Act
    let actual_meters = Meters::new(expected_meters).into_inner();

    // Assert
    assert_eq!(actual_meters, expected_meters);
}

#[test]
fn test_new_accepts_into() {
    // Arrange
    let expected_str = "Bob";

    // Act
    let actual_name = Name::new(expected_str);

    // Assert
    assert_eq!(actual_name.as_ref(), expected_str);
}

#[test]
fn test_from_impl() {
    // Arrange
    let expected_meters_val = 2.0;
    let expected_meters = Meters(expected_meters_val);

    // Act
    let actual_meters: Meters = expected_meters_val.into();

    // Assert
    assert_eq!(actual_meters, expected_meters);
}
