use download::download;

mod books;
mod download;
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
    let download_url = books::fetch_download_url_for(&token, &books[0].data[0], &client).await?;
    download(&download_url, &books[0].data[0], &client).await?;
    Ok(())
}
