use reqwest::header::{HeaderMap, HeaderValue};

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Book {
    pub data: Vec<BookInfo>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct BookInfo {
    #[serde(rename = "productId")]
    pub product_id: String,

    #[serde(rename = "productName")]
    pub product_name: String,
}

pub async fn fetch_books(
    token: &str,
    client: &reqwest::Client,
) -> Result<Vec<Book>, reqwest::Error> {
    // TODO: make offset and limit configurable
    // TODO: take latest from the list and not only one...
    // HACK: print the list and let user choiches
    let url = "https://services.packtpub.com/entitlements-v1/users/me/products?sort=releaseDate:DESC&offset=0&limit=5";
    let req = client.get(url).headers(get_headers(token)).send().await?;
    let t = req.json::<Book>().await?;

    Ok(vec![t])
}

#[derive(serde::Deserialize)]
struct DownloadUrl {
    data: String,
}

pub async fn fetch_download_url_for(
    token: &str,
    book_info: &BookInfo,
    client: &reqwest::Client,
) -> Result<String, reqwest::Error> {
    // TODO: make pdf configurable?
    let url = format!(
        "https://services.packtpub.com/products-v1/products/{}/files/pdf",
        book_info.product_id
    );
    let headers = get_headers(token);
    let res = client.get(url).headers(headers).send().await?;
    let t = res.json::<DownloadUrl>().await?;

    Ok(t.data)
}

fn get_headers(token: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", HeaderValue::from_static("Mozilla/5.0 (Windows NT 6.3; WOW64) AppleWebKit/537.36 KHTML, like Gecko) Chrome/51.0.2704.103 Safari/537.36"));
    headers.insert("Authorization", HeaderValue::from_str(&token).unwrap());
    headers
}
