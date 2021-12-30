use crate::books::BookInfo;
use futures_util::StreamExt;
use tokio::io::AsyncWriteExt;

pub async fn download(
    url: &str,
    book_info: &BookInfo,
    client: &reqwest::Client,
) -> Result<(), reqwest::Error> {
    let res = client.get(url).send().await?;
    let size = res.content_length().unwrap();
    let filename = format!("{}.pdf", book_info.product_name);
    let mut file = tokio::fs::File::create(filename.clone())
        .await
        .expect("error creating file");
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item
            .map_err(|_| "error while downloading file".to_string())
            .unwrap();

        file.write_all(&chunk).await.unwrap();
        downloaded = std::cmp::min(downloaded + (chunk.len() as u64), size);
    }

    Ok(())
}
