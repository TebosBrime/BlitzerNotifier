use anyhow::bail;
use crate::model::{ApiResponse, BlitzerClientRequestParams};
use reqwest_middleware::ClientBuilder;
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};

const BASE_URL: &str = "https://cdn2.atudo.net/api/4.0/pois.php?";

pub async fn get_blitzer_api_result(
    client_params: &BlitzerClientRequestParams,
) -> anyhow::Result<ApiResponse> {
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
    let reqwest_client = ClientBuilder::new(reqwest::Client::new())
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();

    let response = match reqwest_client.get(BASE_URL).query(&client_params.as_query_parameter()).send().await {
        Ok(response) => response,
        Err(err) => bail!("Failed to get blitzer response: {:?}", err),
    };

    if !response.status().is_success() {
        bail!("Request failed with status: {}", response.status());
    }

    match response.json::<ApiResponse>().await {
        Ok(response) => Ok(response),
        Err(error) => {
            bail!("Failed to parse response: {}", error)
        },
    }
}
