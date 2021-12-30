use crate::books::BookInfo;
use futures_util::StreamExt;
use std::io::Write;

pub async fn download(
    url: &str,
    book_info: &BookInfo,
    client: &reqwest::Client,
) -> Result<(), reqwest::Error> {
    let res = client.get(url).send().await?;
    let size = res.content_length().unwrap();
    let filename = format!("{}.pdf", book_info.product_name);
    let mut file = std::fs::File::create(filename)
        .or(Err("failed to create file"))
        .unwrap();
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item
            .or(Err(format!("error while downloading file")))
            .unwrap();
        file.write(&chunk)
            .or(Err(format!("error while writing to file")))
            .unwrap();
        downloaded = std::cmp::min(downloaded + (chunk.len() as u64), size);
    }

    Ok(())
}
