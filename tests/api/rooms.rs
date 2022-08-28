use crate::helpers::spawn_app;

#[tokio::test]
async fn add_room_successful() {
    // Arrange
    let app = spawn_app().await;

    let body = r#"
    {
        "name": "Гостинная"
    }
    "#;

    // Act
    let response = app.add_room(body.into()).await;

    // Assert
    assert_eq!(201, response.status().as_u16());
}