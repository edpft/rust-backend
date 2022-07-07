use rocket::{http::Status, local::blocking::Client};

#[test]
fn test_health_check() {
    // Arrange
    let client = Client::tracked(super::rocket()).unwrap();

    // Act
    let response = client.get("/health_check").dispatch();

    // Assert
    assert_eq!(response.status(), Status::Ok);
    assert!(response.body().is_none());
}
