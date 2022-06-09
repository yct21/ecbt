use std::borrow::Borrow;

use ecbt::{
    prelude::{
        market_pair::{Currency, MarketPair},
        GetPriceTickerRequest,
    },
    Ecbt,
};
use ecbt_exchange::ExchangeMarketData;
use ecbt_huobi::HuobiHttp;

#[tokio::main]
async fn main() {
    let ecbt = Ecbt::http::<HuobiHttp>(()).await.unwrap();
    let request = GetPriceTickerRequest {
        market_pair: MarketPair(Currency::ETH, Currency::USDT),
    };
    let s = ecbt.get_price_ticker(request.borrow()).await.unwrap();
    println!("{:?}", s);
}
