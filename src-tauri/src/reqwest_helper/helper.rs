use reqwest::{header, Client, Response};

use super::error::ReqwestError;

pub async fn get_with_query_params<ResponseType: for<'de> serde::Deserialize<'de>, ReturnType>(
    client: &Client,
    url: String,
    f: fn(ResponseType) -> ReturnType,
    query_params: &Vec<(String, String)>,
) -> Result<ReturnType, ReqwestError> {
    let result = client.get(url).query(query_params).send().await;

    parse_result(result, f).await
}

pub async fn get<ResponseType: for<'de> serde::Deserialize<'de>, ReturnType>(
    client: &Client,
    url: String,
    f: fn(ResponseType) -> ReturnType,
) -> Result<ReturnType, ReqwestError> {
    let result = client.get(url).send().await;

    parse_result(result, f).await
}

pub async fn post<ResponseType: for<'de> serde::Deserialize<'de>, ReturnType>(
    client: &Client,
    url: String,
    body: String,
    f: fn(ResponseType) -> ReturnType,
) -> Result<ReturnType, ReqwestError> {
    let result = client.post(url).body(body).send().await;

    parse_result(result, f).await
}

pub async fn patch<ResponseType: for<'de> serde::Deserialize<'de>, ReturnType>(
    client: &Client,
    url: String,
    body: String,
    content_type: String,
    f: fn(ResponseType) -> ReturnType,
) -> Result<ReturnType, ReqwestError> {
    let result = client
        .patch(url)
        .header(header::CONTENT_TYPE, content_type)
        .body(body)
        .send()
        .await;

    parse_result(result, f).await
}

async fn parse_result<ResponseType: for<'de> serde::Deserialize<'de>, ReturnType>(
    result: Result<Response, reqwest::Error>,
    f: fn(ResponseType) -> ReturnType,
) -> Result<ReturnType, ReqwestError> {
    let response = match result {
        Ok(res) => res,
        Err(e) => return Err(ReqwestError::ReqwestError(e)),
    };

    match response.error_for_status() {
        Ok(res) => match res.json::<ResponseType>().await {
            Ok(parsed) => Ok(f(parsed)),
            Err(e) => {
                println!("{:#?}", e);
                Err(ReqwestError::ResponseJsonParseError(e))
            }
        },
        Err(e) => {
            if e.status() == Some(reqwest::StatusCode::UNAUTHORIZED) {
                Err(ReqwestError::Unauthorized)
            } else {
                Err(ReqwestError::ErrorStatusCode(e))
            }
        }
    }
}
