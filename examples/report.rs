const BASE_URL: &'static str = "http://localhost:6537";

#[tokio::main]
async fn main() {
    let room1 = r#"
    {
        "name": "Dinning Room"
    }
    "#;

    let room2 = r#"
    {
        "name": "Bathroom"
    }
    "#;

    let power_switch1 = r#"
    {
        "description": "Right power switch",
        "name": "Power Switch 1",
        "power_consumption": 224.1,
        "room_name": "Dinning Room"
    }
    "#;

    let power_switch2 = r#"
    {
        "description": "Left power switch",
        "name": "Power Switch 2",
        "power_consumption": 235.1,
        "room_name": "Bathroom"
    }
    "#;

    let thermometer1 = r#"
    {
        "description": "Left thermometer",
        "name": "Thermometer 1",
        "temperature": 23.1,
        "room_name": "Dinning Room"
    }
    "#;

    let thermometer2 = r#"
    {
        "description": "Right thermometer",
        "name": "Thermometer 2",
        "temperature": 33.1,
        "room_name": "Bathroom"
    }
    "#;

    let client = reqwest::Client::new();

    post(&client, "/rooms", room1.to_string()).await;
    post(&client, "/power_switches", power_switch1.to_string()).await;
    post(&client, "/thermometers", thermometer1.to_string()).await;

    post(&client, "/rooms", room2.to_string()).await;
    post(&client, "/power_switches", power_switch2.to_string()).await;
    post(&client, "/thermometers", thermometer2.to_string()).await;

    println!("Report:\n{}", get_report(&client).await);
}

async fn post(client: &reqwest::Client, path: &str, body: String) {
    client
        .post(&format!("{}/api/v1{}", BASE_URL, path))
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .expect(&format!("Failed to execute request {}", path));
}

async fn get_report(client: &reqwest::Client) -> String {
    client
        .get(&format!("{}/api/v1/reports/all", BASE_URL))
        .send()
        .await
        .expect("Failed to receive report")
        .text()
        .await
        .expect("Failed to parse report")
}
