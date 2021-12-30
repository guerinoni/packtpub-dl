use futures_util::StreamExt;
use std::io::Write;

mod books;
mod user;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("expected username password as arguments");
        return Ok(());
    }

    let client = reqwest::Client::new();
    let token = user::fetch_token(args, &client).await?;
    let books = books::fetch_books(&token, &client).await?;
    let download_url =
        books::fetch_download_url_for(&token, &books[0].data[0].product_id, &client).await?;
    let response = client
        .get(download_url)
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
