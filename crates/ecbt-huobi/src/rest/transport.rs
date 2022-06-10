use ecbt_exchange::EcbtError;
use reqwest::{Response, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
use url::Url;

use super::models::ErrorMessage;
use crate::authentication::{Auth, Credentials};
use crate::utils::Result;

pub(super) struct Transport {
    base_url: String,
    auth: Auth,
    client: reqwest::Client,
}

impl Transport {
    pub(crate) fn new(base_url: String, auth: Auth) -> Result<Self> {
        let client = reqwest::Client::builder().build()?; // TODO: check reqwest API
        Ok(Self {
            base_url,
            auth,
            client,
        })
    }

    pub async fn get<O, S>(&self, endpoint: &str, query: Option<&S>, _signed: bool) -> Result<O>
    where
        O: DeserializeOwned,
        S: Serialize + ?Sized,
    {
        // TODO: handle signed

        let mut request = self.client.get(endpoint);
        if let Some(query) = query {
            request = request.query(query);
        }
        let response = request.send().await?;

        self.handle_response(response).await
    }

    pub async fn credential(&self) -> Result<&Credentials> {
        match self.auth {
            Auth::WithCredentials(ref credential) => Ok(credential),
            Auth::WithoutCredentials => Err(EcbtError::NoApiKeySet()),
        }
    }

    // fn build_url<Q>(&self, endpoint: &str, params: Option<&Q>) -> Result<Url>
    // where
    //     Q: Serialize,
    // {
    //     let url = format!("{}{}", self.base_url, endpoint);

    //     let mut url = Url::parse(&url)?;

    //     if params.is_some() {
    //         let query = serde_urlencoded::to_string(params)?;
    //         url.set_query(Some(&query));
    //     };
    //     // if add_recv_window {
    //     //     url.query_pairs_mut()
    //     //         .append_pair("timestamp", &utc_now().to_string());
    //     //     url.query_pairs_mut()
    //     //         .append_pair("recvWindow", &self.recv_window.to_string());
    //     // };
    //     Ok(url)
    // }

    async fn handle_response<O>(&self, response: Response) -> Result<O>
    where
        O: DeserializeOwned,
    {
        match response.status() {
            StatusCode::OK => Ok(response.json::<O>().await?),
            StatusCode::INTERNAL_SERVER_ERROR => Err(EcbtError::InternalServerError()),
            StatusCode::SERVICE_UNAVAILABLE => Err(EcbtError::ServiceUnavailable()),
            StatusCode::UNAUTHORIZED => Err(EcbtError::Unauthorized()),
            StatusCode::BAD_REQUEST => {
                let error: ErrorMessage = response.json().await?;

                Err(EcbtError::Generic(Box::new(error)))
            }
            s => Err(EcbtError::UnkownResponse(format!(
                "Received response: {:?}",
                s
            ))),
        }
    }
}
