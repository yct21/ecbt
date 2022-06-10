use serde::{Deserialize, Serialize};

/// Latest Aggregated Ticker request
///
/// https://huobiapi.github.io/docs/spot/v1/en/#get-latest-aggregated-ticker
#[derive(Clone, Debug, Serialize)]
pub(crate) struct AggregatedTickerRequest {
    pub(crate) symbol: String,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct AggregatedTickerResponse {
    pub(crate) tick: Tick,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Tick {
    pub(crate) close: f64,
}
