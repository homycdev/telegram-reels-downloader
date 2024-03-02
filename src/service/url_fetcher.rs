use crate::domain::types::types::{IGResponse, ReelUrl};
use crate::service::errors::IgFetchError;

pub async fn fetch_url(url: String) -> Result<String, IgFetchError> {
    let url = ReelUrl::new(url);
    let result = reqwest::get(&*url.0).await?.json::<IGResponse>().await;
    match result {
        Ok(value) => {
            log::info!("Successful result received");
            Ok(IGResponse::get_download_url(value))
        }
        Err(e) => {
            log::error!("Recieved error: {}", e.to_string());
            Err(IgFetchError::from_string(e.to_string()))
        }
    }
}


