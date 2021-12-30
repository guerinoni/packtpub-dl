use futures_util::StreamExt;
use reqwest::header::{HeaderMap, HeaderValue};
use std::io::Write;

mod user;

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
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("expected username password as arguments");
        return Ok(());
    }

    let client = reqwest::Client::new();
    let token = user::fetch_token(args, &client).await?;
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", HeaderValue::from_static("Mozilla/5.0 (Windows NT 6.3; WOW64) AppleWebKit/537.36 KHTML, like Gecko) Chrome/51.0.2704.103 Safari/537.36"));
    headers.insert("Authorization", HeaderValue::from_str(&token).unwrap());

    // TODO: make offset and limit configurable
    // TODO: take latest from the list and not only one...
    // HACK: print the list and let user choiches
    let url = "https://services.packtpub.com/entitlements-v1/users/me/products?sort=createdAt:DESC&offset=0&limit=1";
    let req = client
        .get(url)
        .headers(headers.clone())
        .send()
        .await
        .map_err(|e| e.to_string())
        .unwrap();
    let t = req.json::<Book>().await.map_err(|e| e.to_string()).unwrap();

    // TODO: make pdf configurable?
    let url = format!(
        "https://services.packtpub.com/products-v1/products/{}/files/pdf",
        t.data[0].product_id
    );
    let res = client
        .get(url)
        .headers(headers)
        .send()
        .await
        .map_err(|e| e.to_string())
        .unwrap();
    let t = res
        .json::<DownloadUrl>()
        .await
        .map_err(|e| e.to_string())
        .unwrap();

    let response = client
        .get(t.data)
        .send()
        .await
        .map_err(|e| e.to_string())
        .unwrap();
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
