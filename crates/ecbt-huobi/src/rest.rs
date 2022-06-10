mod exchange;
mod market_data;
pub mod models;
mod transport;

use self::transport::Transport;
use crate::utils::Result;
use ecbt_exchange::info::ExchangeInfo;

/// Huobi exchange using HTTP
pub struct HuobiHttp {
    pub exchange_info: ExchangeInfo,
    pub client: BaseClient,
}

/// Internal HTTP client can be retrieve with `Exchange::inner_client`.
pub struct BaseClient {
    transport: Transport,
}

impl BaseClient {
    fn new(url: String) -> Result<Self> {
        Ok(Self {
            // TODO: credentials
            transport: Transport::new(url, crate::authentication::Auth::WithoutCredentials)?,
        })
    }
}
