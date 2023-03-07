pub async fn get_car_info(make: String, model: String) -> serde_json::Value {
    let key = "EsmMfLccdXl79pmJrDLlGg==R0qQarCWKxwLtpHS";
    let client = reqwest::Client::new();
    let url = format!("https://api.api-ninjas.com/v1/cars?&make={}&model={}", make, model);
    let res = client.get(url)
    .header("X-Api-Key", key)
    .send()
    .await
    .unwrap()
    .text()
    .await
    .unwrap();

    let response: serde_json::Value = serde_json::from_str(&res).unwrap();
    response
}