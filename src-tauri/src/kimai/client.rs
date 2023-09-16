use reqwest::{header, Client};

pub struct KimaiClient(pub Client);

impl KimaiClient {
    pub fn get(&self) -> &Self {
        self
    }
}

pub fn configure_kimai_httpclient(user: &str, token: &str) -> Client {
    let mut headers = header::HeaderMap::new();

    let user_value = header::HeaderValue::from_str(user).unwrap();
    headers.insert("X-AUTH-USER", user_value);

    let mut auth_value = header::HeaderValue::from_str(token).unwrap();
    auth_value.set_sensitive(true);

    headers.insert("X-AUTH-TOKEN", auth_value);
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );

    match Client::builder().default_headers(headers).build() {
        Ok(client) => client,
        Err(_) => panic!("Could not build httpClientPool"),
    }
}
