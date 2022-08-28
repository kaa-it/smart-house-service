use crate::helpers::spawn_app;

#[tokio::test]
async fn add_thermometer_successful() {
    // Arrange
    let app = spawn_app().await;

    let room_body = r#"
    {
        "name": "Гостинная"
    }
    "#;

    let thermometer_body = r#"
    {
        "description": "Правый термометер",
        "name": "Термометер 1",
        "temperature": 23.1,
        "room_name": "Гостинная"
    }
    "#;

    // Act
    let _response = app.add_room(room_body.into()).await;
    let response = app.add_thermometer(thermometer_body.into()).await;

    // Assert
    assert_eq!(201, response.status().as_u16());
}

#[tokio::test]
async fn add_thermometer_failed_if_already_exists() {
    // Arrange
    let app = spawn_app().await;

    let room_body = r#"
    {
        "name": "Гостинная"
    }
    "#;

    let thermometer_body = r#"
    {
        "description": "Правый термометер",
        "name": "Термометер 1",
        "temperature": 23.1,
        "room_name": "Гостинная"
    }
    "#;

    // Act
    let _response = app.add_room(room_body.into()).await;
    let _response = app.add_thermometer(thermometer_body.into()).await;
    let response = app.add_thermometer(thermometer_body.into()).await;

    // Assert
    assert_eq!(409, response.status().as_u16());
}

#[tokio::test]
async fn add_thermometer_failed_if_room_not_found() {
    // Arrange
    let app = spawn_app().await;

    let room_body = r#"
    {
        "name": "Гостинная"
    }
    "#;

    let thermometer_body = r#"
    {
        "description": "Правый термометер",
        "name": "Термометер 1",
        "temperature": 23.1,
        "room_name": "Гостинная2"
    }
    "#;

    // Act
    let _response = app.add_room(room_body.into()).await;
    let response = app.add_thermometer(thermometer_body.into()).await;

    // Assert
    assert_eq!(404, response.status().as_u16());
}

#[tokio::test]
async fn remove_thermometer_successful() {
    // Arrange
    let app = spawn_app().await;

    let room_body = r#"
    {
        "name": "Гостинная"
    }
    "#;

    let thermometer_body = r#"
    {
        "description": "Правый термометер",
        "name": "Термометер 1",
        "temperature": 23.1,
        "room_name": "Гостинная"
    }
    "#;

    let remove_thermometer_body = r#"
    {
        "name": "Термометер 1",
        "room_name": "Гостинная"
    }
    "#;

    // Act
    let _response = app.add_room(room_body.into()).await;
    let _response = app.add_thermometer(thermometer_body.into()).await;
    let response = app.remove_thermometer(remove_thermometer_body.into()).await;

    // Assert
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn remove_thermometer_failed_if_not_found() {
    // Arrange
    let app = spawn_app().await;

    let remove_thermometer_body = r#"
    {
        "name": "Термометер 1",
        "room_name": "Гостинная"
    }
    "#;

    // Act
    let response = app.remove_thermometer(remove_thermometer_body.into()).await;

    // Assert
    assert_eq!(404, response.status().as_u16());
}
