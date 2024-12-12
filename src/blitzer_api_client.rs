use reqwest_middleware::{ClientBuilder};
use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};
use crate::model::{ApiResponse, BlitzerClientRequestParams};

const BASE_URL: &str = "https://cdn2.atudo.net/api/4.0/pois.php";

pub async fn get_blitzer_api_result(client_params: BlitzerClientRequestParams) -> anyhow::Result<ApiResponse> {
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
    let reqwest_client = ClientBuilder::new(reqwest::Client::new())        
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();

    let url = format!("{}{}", BASE_URL, client_params.get_request_params());
    let response = reqwest_client
        .get(url)
        .send().await?;

    if !response.status().is_success() {
        eprintln!("Request failed with status: {}", response.status());
    }

    let response = response.json().await?;
    Ok(response)
}

