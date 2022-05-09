use axum::{
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Json,
};
use serde::{Deserialize, Serialize};
use stripe::{
    CheckoutSession, CheckoutSessionMode, Client, CreateCheckoutSession,
    CreateCheckoutSessionLineItems, CreateCheckoutSessionLineItemsPriceData,
    CreateCheckoutSessionLineItemsPriceDataProductData, Currency,
};

pub async fn handle_single_stripe_payment(
    data: Json<HandleStripePaymentBody>,
) -> impl IntoResponse {
    let client_secret: &str = env!("STRIPE_CLIENT_SECRET");
    let client = Client::new(client_secret);
    let mut checkout_session_params =
        CreateCheckoutSession::new(&data.cancel_uri, &data.success_uri);
    let mut line_item = CreateCheckoutSessionLineItems::default();
    let mut price_data = CreateCheckoutSessionLineItemsPriceData::default();
    let mut product_data = CreateCheckoutSessionLineItemsPriceDataProductData::default();
    product_data.name = "Realy cool course".to_owned();
    price_data.currency = Currency::USD;
    price_data.product_data = Some(product_data);
    price_data.unit_amount = Some(1);
    line_item.price_data = Some(price_data);
    line_item.quantity = Some(1);
    checkout_session_params.line_items = Some(vec![line_item]);
    checkout_session_params.mode = Some(CheckoutSessionMode::Payment);
    let checkout_session = match CheckoutSession::create(&client, checkout_session_params).await {
        Ok(checkout_session) => checkout_session,
        Err(error) => {
            tracing::error!("{}", error.to_string());
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Redirect::to(&data.cancel_uri),
            );
        }
    };
    let redirect_uri = checkout_session.url.unwrap();
    (StatusCode::OK, Redirect::to(&redirect_uri))
}

#[derive(Deserialize, Serialize)]
pub struct HandleStripePaymentBody {
    cancel_uri: String,
    success_uri: String,
}
