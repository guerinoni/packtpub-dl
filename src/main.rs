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

#[derive(serde::Deserialize)]
struct UserInfo {
    data: Data,
}

#[derive(serde::Deserialize)]
struct Data {
    access: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let email = "guerinoni.federico@gmail.com";
    let password = "frac.maff1juft9VEEP";
    // let header = "User-Agent: Mozilla/5.0 (Windows NT 6.3; WOW64) AppleWebKit/537.36 KHTML, like Gecko) Chrome/51.0.2704.103 Safari/537.36, Authorization:";

    let url = "https://services.packtpub.com/auth-v1/users/tokens";
    let mut map = std::collections::HashMap::new();
    map.insert("username", email);
    map.insert("password", password);
    let client = reqwest::Client::new();
    let post = client.post(url).json(&map).send().await?;
    let j = post.json::<UserInfo>().await?;
    println!("token ok {}", j.data.access);
    Ok(())
}
