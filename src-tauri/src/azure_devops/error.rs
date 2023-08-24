use thiserror::Error;

#[derive(Error, Debug)]
pub enum AzureDevopsError {
    #[error("reqwest Error")]
    ReqwestError(reqwest::Error),
    #[error("authentication not valid")]
    Unauthorized,
    #[error("error statuscode was returned")]
    ErrorStatusCode(reqwest::Error),
    #[error("json parsing was not successful")]
    ResponseJsonParseError(reqwest::Error),
}

impl serde::Serialize for AzureDevopsError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
