use std::collections::HashMap;

#[derive(serde::Deserialize)]
struct UserInfo {
    data: Data,
}

#[derive(serde::Deserialize)]
struct Data {
    access: String,
}

pub async fn fetch_token(
    args: Vec<String>,
    client: &reqwest::Client,
) -> Result<String, reqwest::Error> {
    let url = "https://services.packtpub.com/auth-v1/users/tokens";
    let hm = HashMap::from([
        ("username", args[1].as_str()),
        ("password", args[2].as_str()),
    ]);

    let res = client.post(url).json(&hm).send().await?;
    let info = res.json::<UserInfo>().await?;
    let token = String::from("Bearer ") + info.data.access.as_str();
    Ok(token)
}
