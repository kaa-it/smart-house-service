use crate::helpers::spawn_app;

#[tokio::test]
async fn add_power_switch_successful() {
    // Arrange
    let app = spawn_app().await;

    let room_body = r#"
    {
        "name": "Гостинная"
    }
    "#;

    let power_switch_body = r#"
    {
        "description": "Правая розетка",
        "name": "Розетка 1",
        "power_consumption": 224.1,
        "room_name": "Гостинная"
    }
    "#;

    // Act
    let _response = app.add_room(room_body.into()).await;
    let response = app.add_power_switch(power_switch_body.into()).await;

    // Assert
    assert_eq!(201, response.status().as_u16());
}

#[tokio::test]
async fn add_power_switch_failed_if_already_exists() {
    // Arrange
    let app = spawn_app().await;

    let room_body = r#"
    {
        "name": "Гостинная"
    }
    "#;

    let power_switch_body = r#"
    {
        "description": "Правая розетка",
        "name": "Розетка 1",
        "power_consumption": 224.1,
        "room_name": "Гостинная"
    }
    "#;

    // Act
    let _response = app.add_room(room_body.into()).await;
    let _response = app.add_power_switch(power_switch_body.into()).await;
    let response = app.add_power_switch(power_switch_body.into()).await;


    // Assert
    assert_eq!(409, response.status().as_u16());
}

#[tokio::test]
async fn add_power_switch_failed_if_room_not_found() {
    // Arrange
    let app = spawn_app().await;

    let room_body = r#"
    {
        "name": "Гостинная"
    }
    "#;

    let power_switch_body = r#"
    {
        "description": "Правая розетка",
        "name": "Розетка 1",
        "power_consumption": 224.1,
        "room_name": "Гостинная2"
    }
    "#;

    // Act
    let _response = app.add_room(room_body.into()).await;
    let response = app.add_power_switch(power_switch_body.into()).await;

    // Assert
    assert_eq!(404, response.status().as_u16());
}

#[tokio::test]
async fn remove_power_switch_successful() {
    // Arrange
    let app = spawn_app().await;

    let room_body = r#"
    {
        "name": "Гостинная"
    }
    "#;

    let power_switch_body = r#"
    {
        "description": "Правая розетка",
        "name": "Розетка 1",
        "power_consumption": 224.1,
        "room_name": "Гостинная"
    }
    "#;

    let remove_power_switch_body = r#"
    {
        "name": "Розетка 1",
        "room_name": "Гостинная"
    }
    "#;

    // Act
    let _response = app.add_room(room_body.into()).await;
    let _response = app.add_power_switch(power_switch_body.into()).await;
    let response = app.remove_power_switch(remove_power_switch_body.into()).await;

    // Assert
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn remove_power_switch_failed_if_not_found() {
    // Arrange
    let app = spawn_app().await;

    let remove_power_switch_body = r#"
    {
        "name": "Розетка 1",
        "room_name": "Гостинная"
    }
    "#;

    // Act
    let response = app.remove_power_switch(remove_power_switch_body.into()).await;

    // Assert
    assert_eq!(404, response.status().as_u16());
}
