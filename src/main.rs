// # this is base url where i do the requests
// BASE_URL = "https://services.packtpub.com/"

// # URL to request jwt token, params by post are user and pass, return jwt token
// AUTH_ENDPOINT = "auth-v1/users/tokens"

// # URL to get all your books, two params that i change are offset and limit, method GET
// PRODUCTS_ENDPOINT = "entitlements-v1/users/me/products?sort=createdAt:DESC&offset={offset}&limit={limit}"

// # URL to get types , param is  book id, method GET
// URL_BOOK_TYPES_ENDPOINT = "products-v1/products/{book_id}/types"

// # URL to get url file to download, params are book id and format of the file (can be pdf, epub, etc..), method GET
// URL_BOOK_ENDPOINT = "products-v1/products/{book_id}/files/{format}"

use futures_util::StreamExt;
use std::io::Write;

#[derive(serde::Deserialize)]
struct UserInfo {
    data: Data,
}

#[derive(serde::Deserialize)]
struct Data {
    access: String,
}

#[derive(serde::Deserialize)]
struct Book {
    data: Vec<BookInfo>,
}

#[derive(serde::Deserialize)]
struct BookInfo {
    #[serde(rename = "productId")]
    product_id: String,
}

#[derive(serde::Deserialize)]
struct DownloadUrl {
    data: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let email = "guerinoni.federico@gmail.com";
    let password = "frac.maff1juft9VEEP";
    let url = "https://services.packtpub.com/auth-v1/users/tokens";
    let mut map = std::collections::HashMap::new();
    map.insert("username", email);
    map.insert("password", password);
    let client = reqwest::Client::new();
    let post = client.post(url).json(&map).send().await?;
    let j = post.json::<UserInfo>().await?;
    let mut token = String::from("Bearer ");
    token.push_str(j.data.access.as_str());

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("User-Agent", reqwest::header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 6.3; WOW64) AppleWebKit/537.36 KHTML, like Gecko) Chrome/51.0.2704.103 Safari/537.36"));
    headers.insert(
        "Authorization",
        reqwest::header::HeaderValue::from_str(&token).unwrap(),
    );

    // TODO: make offset and limit configurable
    let url = "https://services.packtpub.com/entitlements-v1/users/me/products?sort=createdAt:DESC&offset=0&limit=1";
    let req = client.get(url).headers(headers.clone()).send().await?;
    let t = req.json::<Book>().await?;

    // TODO: make pdf configurable?
    let url = format!(
        "https://services.packtpub.com/products-v1/products/{}/files/pdf",
        t.data[0].product_id
    );
    let res = client.get(url).headers(headers).send().await?;
    let t = res.json::<DownloadUrl>().await?;

    let response = client.get(t.data).send().await?;
    let size = response
        .content_length()
        .ok_or("failed to get content size")
        .unwrap();
    let mut file = std::fs::File::create("book.pdf")
        .or(Err("failed to create file"))
        .unwrap();
    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item
            .or(Err(format!("Error while downloading file")))
            .unwrap();
        file.write(&chunk)
            .or(Err(format!("Error while writing to file")))
            .unwrap();
        let new = std::cmp::min(downloaded + (chunk.len() as u64), size);
        downloaded = new;
    }

    Ok(())
}
