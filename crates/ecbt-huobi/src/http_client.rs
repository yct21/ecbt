mod exchange;
mod transport;

use self::transport::Transport;
use ecbt_exchange::info::ExchangeInfo;

/// Huobi exchange using HTTP
pub struct HuobiHttp {
    pub exchange_info: ExchangeInfo,
    pub client: BaseClient,
}

pub struct BaseClient {
    transport: Transport,
}

impl BaseClient {
    fn new(url: String) -> Self {
        Self {
            transport: Transport::new(url),
        }
    }
}
