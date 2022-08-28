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

#[tokio::test]
async fn add_room_failed_if_already_exists() {
    // Arrange
    let app = spawn_app().await;

    let body = r#"
    {
        "name": "Гостинная"
    }
    "#;

    // Act
    let _response = app.add_room(body.into()).await;
    let response = app.add_room(body.into()).await;

    // Assert
    assert_eq!(409, response.status().as_u16());
}

#[tokio::test]
async fn remove_room_failed_if_not_found() {
    // Arrange
    let app = spawn_app().await;

    let body = r#"
    {
        "name": "Гостинная"
    }
    "#;

    // Act
    let response = app.remove_room(body.into()).await;

    // Assert
    assert_eq!(404, response.status().as_u16());
}

#[tokio::test]
async fn remove_room_successful() {
    // Arrange
    let app = spawn_app().await;

    let body = r#"
    {
        "name": "Гостинная"
    }
    "#;

    // Act
    let _response = app.add_room(body.into()).await;
    let response = app.remove_room(body.into()).await;

    // Assert
    assert_eq!(200, response.status().as_u16());
}
