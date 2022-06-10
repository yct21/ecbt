//! Huobi market data API

use std::collections::HashMap;

use crate::utils::Result;

use super::{
    models::aggregated_ticker::{AggregatedTickerRequest, AggregatedTickerResponse},
    HuobiHttp,
};

impl HuobiHttp {
    pub(crate) async fn get_aggregated_ticker(
        &self,
        symbol: String,
    ) -> Result<AggregatedTickerResponse> {
        let query = AggregatedTickerRequest { symbol };
        let result = self
            .client
            .transport
            .get("/market/detail/merged", Some(&query), false)
            .await?;

        Ok(result)
    }
}
