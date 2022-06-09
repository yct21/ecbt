//! adaptor for ecbt-exchange

use async_trait::async_trait;
use ecbt_exchange::{
    info::{ExchangeInfo, ExchangeInfoRetrieval, MarketPairHandle, MarketPairInfo},
    model::{
        market_pair::MarketPair, Balance, CancelAllOrdersRequest, CancelOrderRequest, Candle,
        EcbtOrderRequest, GetHistoricRatesRequest, GetHistoricTradesRequest,
        GetOrderHistoryRequest, GetOrderRequest, GetPriceTickerRequest, OpenMarketOrderRequest,
        Order, OrderBookRequest, OrderBookResponse, OrderCanceled, Paginator, Ticker, Trade,
        TradeHistoryRequest,
    },
    shared::Result,
    Exchange, ExchangeAccount, ExchangeMarketData,
};

use super::{BaseClient, HuobiHttp};

#[async_trait]
impl Exchange for HuobiHttp {
    type InitParams = (); // TODO: use real parameter
    type InnerClient = BaseClient;

    async fn new(_params: Self::InitParams) -> Result<Self> {
        let exchange_info = ExchangeInfo::new();
        let base_client = BaseClient::new("https://api.huobi.pro".to_owned());
        let huobi_http = Self {
            exchange_info,
            client: base_client,
        };

        // TODO: refresh market info
        Ok(huobi_http)
    }

    fn inner_client(&self) -> Option<&Self::InnerClient> {
        Some(&self.client)
    }
}

// TODO: fix function signature
#[async_trait]
impl ExchangeInfoRetrieval for HuobiHttp {
    async fn get_pair(&self, market_pair: &MarketPair) -> Result<MarketPairHandle> {
        todo!()
    }
    async fn retrieve_pairs(&self) -> Result<Vec<MarketPairInfo>> {
        todo!()
    }
    async fn refresh_market_info(&self) -> Result<Vec<MarketPairHandle>> {
        todo!()
    }
}

#[async_trait]
impl ExchangeMarketData for HuobiHttp {
    async fn order_book(&self, req: &OrderBookRequest) -> Result<OrderBookResponse> {
        todo!()
    }
    async fn get_price_ticker(&self, req: &GetPriceTickerRequest) -> Result<Ticker> {
        todo!()
    }
    async fn get_historic_rates(&self, req: &GetHistoricRatesRequest) -> Result<Vec<Candle>> {
        todo!()
    }
    async fn get_historic_trades(&self, req: &GetHistoricTradesRequest) -> Result<Vec<Trade>> {
        todo!()
    }
}

#[async_trait]
impl ExchangeAccount for HuobiHttp {
    async fn limit_buy(&self, req: &EcbtOrderRequest) -> Result<Order> {
        todo!()
    }
    async fn limit_sell(&self, req: &EcbtOrderRequest) -> Result<Order> {
        todo!()
    }
    async fn market_buy(&self, req: &OpenMarketOrderRequest) -> Result<Order> {
        todo!()
    }
    async fn market_sell(&self, req: &OpenMarketOrderRequest) -> Result<Order> {
        todo!()
    }
    async fn cancel_order(&self, req: &CancelOrderRequest) -> Result<OrderCanceled> {
        todo!()
    }
    async fn cancel_all_orders(&self, req: &CancelAllOrdersRequest) -> Result<Vec<OrderCanceled>> {
        todo!()
    }
    async fn get_all_open_orders(&self) -> Result<Vec<Order>> {
        todo!()
    }
    async fn get_order_history(&self, req: &GetOrderHistoryRequest) -> Result<Vec<Order>> {
        todo!()
    }
    async fn get_trade_history(&self, req: &TradeHistoryRequest) -> Result<Vec<Trade>> {
        todo!()
    }
    async fn get_account_balances(&self, paginator: Option<Paginator>) -> Result<Vec<Balance>> {
        todo!()
    }
    async fn get_order(&self, req: &GetOrderRequest) -> Result<Order> {
        todo!()
    }
}
