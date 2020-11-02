//! In some contexts, such as bindings in other languages (e.g., Python via pyo3), it is not possible to use trait
//! constraints on generics. This module provides an enum wrapper type for all openlimits exchanges that code can
//! use to operate over any openlimits-supported exchange without generics

use crate::binance::{Binance, BinanceParameters, BinanceWebsocket};
use crate::exchange::{Exchange, ExchangeAccount, ExchangeMarketData};
use crate::exchange_info::{ExchangeInfoRetrieval, MarketPair, MarketPairHandle};
use crate::exchange_ws::{ExchangeWs, OpenLimitsWs};
use crate::nash::{Nash, NashParameters, NashStream};
use crate::{
    model::{
        websocket::{OpenLimitsWebsocketMessage, Subscription},
        Balance, CancelAllOrdersRequest, CancelOrderRequest, Candle, GetHistoricRatesRequest,
        GetHistoricTradesRequest, GetOrderHistoryRequest, GetOrderRequest, GetPriceTickerRequest,
        OpenLimitOrderRequest, OpenMarketOrderRequest, Order, OrderBookRequest, OrderBookResponse,
        OrderCanceled, Paginator, Ticker, Trade, TradeHistoryRequest,
    },
    shared::Result,
};
use async_trait::async_trait;
use futures::stream::{Stream, StreamExt};
use std::{pin::Pin, task::Context, task::Poll};

#[derive(Clone)]
pub enum InitAnyExchange {
    Nash(NashParameters),
    Binance(BinanceParameters),
}

pub enum AnyExchange {
    Nash(Nash),
    Binance(Binance),
}

#[async_trait]
impl Exchange for AnyExchange {
    type InitParams = InitAnyExchange;
    type InnerClient = ();
    async fn new(params: InitAnyExchange) -> Self {
        match params {
            InitAnyExchange::Nash(params) => Nash::new(params).await.into(),
            InitAnyExchange::Binance(params) => Binance::new(params).await.into(),
        }
    }
    // not particularly useful to access the inner client with this type. could wrap the inner
    // client reference in an enum, but that would introduce lifetimes all the way down due to
    // https://users.rust-lang.org/t/how-to-specify-lifetime-for-associated-type/5736
    fn inner_client(&self) -> Option<&Self::InnerClient> {
        None
    }
}

#[async_trait]
impl ExchangeInfoRetrieval for AnyExchange {
    async fn get_pair(&self, name: &str) -> Result<MarketPairHandle> {
        match self {
            Self::Nash(nash) => nash.get_pair(name).await,
            Self::Binance(binance) => binance.get_pair(name).await,
        }
    }
    async fn retrieve_pairs(&self) -> Result<Vec<MarketPair>> {
        match self {
            Self::Nash(nash) => nash.retrieve_pairs().await,
            Self::Binance(binance) => binance.retrieve_pairs().await,
        }
    }
    async fn refresh_market_info(&self) -> Result<Vec<MarketPairHandle>> {
        match self {
            Self::Nash(nash) => nash.refresh_market_info().await,
            Self::Binance(binance) => binance.refresh_market_info().await,
        }
    }
}

#[async_trait]
impl ExchangeAccount for AnyExchange {
    async fn limit_buy(&self, req: &OpenLimitOrderRequest) -> Result<Order> {
        match self {
            Self::Nash(nash) => nash.limit_buy(req).await,
            Self::Binance(binance) => binance.limit_buy(req).await,
        }
    }
    async fn limit_sell(&self, req: &OpenLimitOrderRequest) -> Result<Order> {
        match self {
            Self::Nash(nash) => nash.limit_sell(req).await,
            Self::Binance(binance) => binance.limit_sell(req).await,
        }
    }
    async fn market_buy(&self, req: &OpenMarketOrderRequest) -> Result<Order> {
        match self {
            Self::Nash(nash) => nash.market_buy(req).await,
            Self::Binance(binance) => binance.market_buy(req).await,
        }
    }
    async fn market_sell(&self, req: &OpenMarketOrderRequest) -> Result<Order> {
        match self {
            Self::Nash(nash) => nash.market_sell(req).await,
            Self::Binance(binance) => binance.market_sell(req).await,
        }
    }
    async fn cancel_order(&self, req: &CancelOrderRequest) -> Result<OrderCanceled> {
        match self {
            Self::Nash(nash) => nash.cancel_order(req).await,
            Self::Binance(binance) => binance.cancel_order(req).await,
        }
    }
    async fn cancel_all_orders(&self, req: &CancelAllOrdersRequest) -> Result<Vec<OrderCanceled>> {
        match self {
            Self::Nash(nash) => nash.cancel_all_orders(req).await,
            Self::Binance(binance) => binance.cancel_all_orders(req).await,
        }
    }
    async fn get_all_open_orders(&self) -> Result<Vec<Order>> {
        match self {
            Self::Nash(nash) => nash.get_all_open_orders().await,
            Self::Binance(binance) => binance.get_all_open_orders().await,
        }
    }
    async fn get_order_history(&self, req: &GetOrderHistoryRequest) -> Result<Vec<Order>> {
        match self {
            Self::Nash(nash) => nash.get_order_history(req).await,
            Self::Binance(binance) => binance.get_order_history(req).await,
        }
    }
    async fn get_trade_history(&self, req: &TradeHistoryRequest) -> Result<Vec<Trade>> {
        match self {
            Self::Nash(nash) => nash.get_trade_history(req).await,
            Self::Binance(binance) => binance.get_trade_history(req).await,
        }
    }
    async fn get_account_balances(&self, paginator: Option<Paginator>) -> Result<Vec<Balance>> {
        match self {
            Self::Nash(nash) => nash.get_account_balances(paginator).await,
            Self::Binance(binance) => binance.get_account_balances(paginator).await,
        }
    }
    async fn get_order(&self, req: &GetOrderRequest) -> Result<Order> {
        match self {
            Self::Nash(nash) => nash.get_order(req).await,
            Self::Binance(binance) => binance.get_order(req).await,
        }
    }
}

#[async_trait]
impl ExchangeMarketData for AnyExchange {
    async fn order_book(&self, req: &OrderBookRequest) -> Result<OrderBookResponse> {
        match self {
            Self::Nash(nash) => nash.order_book(req).await,
            Self::Binance(binance) => binance.order_book(req).await,
        }
    }
    async fn get_price_ticker(&self, req: &GetPriceTickerRequest) -> Result<Ticker> {
        match self {
            Self::Nash(nash) => nash.get_price_ticker(req).await,
            Self::Binance(binance) => binance.get_price_ticker(req).await,
        }
    }
    async fn get_historic_rates(&self, req: &GetHistoricRatesRequest) -> Result<Vec<Candle>> {
        match self {
            Self::Nash(nash) => nash.get_historic_rates(req).await,
            Self::Binance(binance) => binance.get_historic_rates(req).await,
        }
    }
    async fn get_historic_trades(&self, req: &GetHistoricTradesRequest) -> Result<Vec<Trade>> {
        match self {
            Self::Nash(nash) => nash.get_historic_trades(req).await,
            Self::Binance(binance) => binance.get_historic_trades(req).await,
        }
    }
}

pub enum AnyWsExchange {
    Nash(OpenLimitsWs<NashStream>),
    Binance(OpenLimitsWs<BinanceWebsocket>),
}

impl Stream for AnyWsExchange {
    type Item = Result<OpenLimitsWebsocketMessage>;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match unsafe { self.get_unchecked_mut() } {
            Self::Nash(nash) => nash.poll_next_unpin(cx),
            Self::Binance(binance) => binance.poll_next_unpin(cx),
        }
    }
}

#[async_trait]
impl ExchangeWs for AnyWsExchange {
    type InitParams = InitAnyExchange;
    async fn new(params: Self::InitParams) -> Self {
        match params {
            InitAnyExchange::Nash(params) => {
                OpenLimitsWs::<NashStream>::instantiate(params).await.into()
            }
            InitAnyExchange::Binance(_) => OpenLimitsWs::<BinanceWebsocket>::instantiate(())
                .await
                .into(),
        }
    }
    async fn subscribe(&mut self, subscription: Subscription) -> Result<()> {
        match self {
            Self::Nash(nash) => nash.subscribe(subscription).await,
            Self::Binance(binance) => binance.subscribe(subscription).await,
        }
    }
    fn parse_message(&self, message: Self::Item) -> Result<OpenLimitsWebsocketMessage> {
        message
    }
}

impl From<Nash> for AnyExchange {
    fn from(nash: Nash) -> Self {
        Self::Nash(nash)
    }
}

impl From<Binance> for AnyExchange {
    fn from(binance: Binance) -> Self {
        Self::Binance(binance)
    }
}

impl From<OpenLimitsWs<NashStream>> for AnyWsExchange {
    fn from(nash: OpenLimitsWs<NashStream>) -> Self {
        Self::Nash(nash)
    }
}

impl From<OpenLimitsWs<BinanceWebsocket>> for AnyWsExchange {
    fn from(binance: OpenLimitsWs<BinanceWebsocket>) -> Self {
        Self::Binance(binance)
    }
}
