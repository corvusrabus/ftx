use super::common::{Id, OrderStatus, OrderType, Side};
use super::Request;
use chrono::{DateTime, Utc};
use http::Method;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderInfo {
    pub id: Id,
    pub market: String,
    pub future: Option<String>,
    pub r#type: OrderType,
    pub side: Side,
    pub price: Option<Decimal>, // null for new market orders
    pub size: Decimal,
    pub reduce_only: Option<bool>,
    pub ioc: Option<bool>,
    pub post_only: Option<bool>,
    pub status: OrderStatus,
    pub filled_size: Option<Decimal>,
    pub remaining_size: Option<Decimal>,
    pub avg_fill_price: Option<Decimal>,
    pub liquidation: Option<bool>,
    pub created_at: DateTime<Utc>,
    pub client_id: Option<String>,
    pub retry_until_filled: Option<bool>,
    pub trigger_price: Option<Decimal>,
    pub order_price: Option<Decimal>,
    pub triggered_at: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct GetOpenOrders<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market: Option<&'a str>,
}

impl<'a> GetOpenOrders<'a> {
    pub fn all_market() -> Self {
        Self { market: None }
    }

    pub fn with_market(market: &'a str) -> Self {
        Self {
            market: Some(market),
        }
    }
}

impl Request for GetOpenOrders<'_> {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/orders";
    const AUTH: bool = true;
    #[cfg(feature = "optimized-access")]
    const OPTIMIZED_ACCESS_SUPPORTED: bool = true;
    type Response = Vec<OrderInfo>;
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrder<'a> {
    pub market: &'a str,
    pub side: Side,
    // Price should be serialized even if it is None, otherwise
    // market orders will break; test with rest::tests::market_order
    pub price: Option<Decimal>,
    pub r#type: OrderType,
    pub size: Decimal,
    pub reduce_only: bool,
    pub ioc: bool,
    pub post_only: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<&'a str>,
    pub reject_on_price_band: bool,
}

impl Request for PlaceOrder<'_> {
    const METHOD: Method = Method::POST;
    const PATH: &'static str = "/orders";
    const AUTH: bool = true;
    #[cfg(feature = "optimized-access")]
    const OPTIMIZED_ACCESS_SUPPORTED: bool = true;
    type Response = OrderInfo;
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ModifyOrder<'a> {
    #[serde(skip_serializing)]
    pub id: Id,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<&'a str>,
}

impl Request for ModifyOrder<'_> {
    const METHOD: Method = Method::POST;
    const PATH: &'static str = "/orders/{}/modify";
    const AUTH: bool = true;

    type Response = OrderInfo;

    fn path(&self) -> Cow<'_, str> {
        Cow::Owned(format!("/orders/{}/modify", self.id))
    }
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct GetOrder {
    #[serde(skip_serializing)]
    pub id: Id,
}

impl GetOrder {
    pub fn new(order_id: Id) -> Self {
        Self { id: order_id }
    }
}

impl Request for GetOrder {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/orders/{}";
    const AUTH: bool = true;

    type Response = OrderInfo;

    fn path(&self) -> Cow<'_, str> {
        Cow::Owned(format!("/orders/{}", self.id))
    }
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct CancelOrder {
    #[serde(skip_serializing)]
    pub id: Id,
}

impl CancelOrder {
    pub fn new(order_id: Id) -> Self {
        Self { id: order_id }
    }
}

impl Request for CancelOrder {
    const METHOD: Method = Method::DELETE;
    const PATH: &'static str = "/orders/{}";
    const AUTH: bool = true;
    #[cfg(feature = "optimized-access")]
    const OPTIMIZED_ACCESS_SUPPORTED: bool = true;
    type Response = String;

    fn path(&self) -> Cow<'_, str> {
        Cow::Owned(format!("/orders/{}", self.id))
    }
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct CancelTriggerOrder {
    #[serde(skip_serializing)]
    pub id: Id,
}

impl CancelTriggerOrder {
    pub fn new(order_id: Id) -> Self {
        Self { id: order_id }
    }
}

impl Request for CancelTriggerOrder {
    const METHOD: Method = Method::DELETE;
    const PATH: &'static str = "/conditional_orders/{}";
    const AUTH: bool = true;
    type Response = String;

    fn path(&self) -> Cow<'_, str> {
        Cow::Owned(format!("/conditional_orders/{}", self.id))
    }
}
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOrder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<Side>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_orders_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_orders_only: Option<bool>,
}

impl<'a> CancelAllOrder<'a> {
    pub fn with_market(market: &'a str) -> Self {
        Self {
            market: Some(market),
            ..Default::default()
        }
    }
}

impl Request for CancelAllOrder<'_> {
    const METHOD: Method = Method::DELETE;
    const PATH: &'static str = "/orders";
    const AUTH: bool = true;

    type Response = String;
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderByClientId<'a> {
    #[serde(skip_serializing)]
    pub client_id: &'a str,
}

impl<'a> CancelOrderByClientId<'a> {
    pub fn new(client_id: &'a str) -> Self {
        Self { client_id }
    }
}

impl Request for CancelOrderByClientId<'_> {
    const METHOD: Method = Method::DELETE;
    const PATH: &'static str = "/orders/by_client_id/{}";
    const AUTH: bool = true;
    #[cfg(feature = "optimized-access")]
    const OPTIMIZED_ACCESS_SUPPORTED: bool = true;
    type Response = String;

    fn path(&self) -> Cow<'_, str> {
        Cow::Owned(format!("/orders/by_client_id/{}", self.client_id))
    }
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderByClientId<'a> {
    #[serde(skip_serializing)]
    pub client_id: &'a str,
}

impl<'a> GetOrderByClientId<'a> {
    pub fn new(client_id: &'a str) -> Self {
        Self { client_id }
    }
}

impl Request for GetOrderByClientId<'_> {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/orders/by_client_id/{}";
    const AUTH: bool = true;

    type Response = OrderInfo;

    fn path(&self) -> Cow<'_, str> {
        Cow::Owned(format!("/orders/by_client_id/{}", self.client_id))
    }
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct GetOrderHistory<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<Side>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::serialize_as_timestamp"
    )]
    pub start_time: Option<DateTime<Utc>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::serialize_as_timestamp"
    )]
    pub end_time: Option<DateTime<Utc>>,
}

impl Request for GetOrderHistory<'_> {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/orders/history";
    const AUTH: bool = true;

    type Response = Vec<OrderInfo>;
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlaceTriggerOrder<'a> {
    pub market: &'a str,
    pub side: Side,
    pub size: Decimal,
    pub r#type: OrderType,
    pub trigger_price: Decimal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_until_filled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_price: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_value: Option<Decimal>,
}

impl Request for PlaceTriggerOrder<'_> {
    const METHOD: Method = Method::POST;
    const PATH: &'static str = "/conditional_orders";
    const AUTH: bool = true;

    type Response = OrderInfo;
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ModifyOrderByClientId<'a> {
    #[serde(skip_serializing)]
    pub client_id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<Decimal>,
}

impl Request for ModifyOrderByClientId<'_> {
    const METHOD: Method = Method::POST;
    const PATH: &'static str = "/orders/by_client_id/{}/modify";
    const AUTH: bool = true;

    type Response = OrderInfo;

    fn path(&self) -> Cow<'_, str> {
        Cow::Owned(format!("/orders/by_client_id/{}/modify", self.client_id))
    }
}
