use base64::{engine::general_purpose, Engine as _};
use reqwest::{header, Client};

pub struct AzureDevopsClient(pub Client);

impl AzureDevopsClient {
    pub fn get(&self) -> &Self {
        self
    }
}

pub fn configure_devops_httpclient(user: &String, pat: &String) -> Client {
    let mut headers = header::HeaderMap::new();

    let base64_auth_value = general_purpose::STANDARD_NO_PAD.encode(format!("{user}:{pat}"));
    let basic_auth_header = format!("Basic {base64_auth_value}");
    let mut auth_value = header::HeaderValue::from_str(basic_auth_header.as_str()).unwrap();
    auth_value.set_sensitive(true);

    headers.insert(header::AUTHORIZATION, auth_value);
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );

    match Client::builder().default_headers(headers).build() {
        Ok(client) => client,
        Err(_) => panic!("Could not build httpClientPool"),
    }
}
