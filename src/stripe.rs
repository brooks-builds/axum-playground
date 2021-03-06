use axum::{
    body::Bytes,
    extract::Form,
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use dotenv_codegen::dotenv;
use headers::HeaderMap;
use serde::{Deserialize, Serialize};
use stripe::{
    CheckoutSession, CheckoutSessionMode, Client, CreateCheckoutSession,
    CreateCheckoutSessionLineItems, CreateCheckoutSessionLineItemsPriceData,
    CreateCheckoutSessionLineItemsPriceDataProductData, Currency, Webhook,
};

pub async fn handle_single_stripe_payment(
    Form(data): Form<HandleStripePaymentBody>,
) -> impl IntoResponse {
    let client_secret: &str = dotenv!("STRIPE_CLIENT_SECRET");
    let client = Client::new(client_secret);
    let mut checkout_session_params =
        CreateCheckoutSession::new(&data.cancel_uri, &data.success_uri);
    let mut line_item = CreateCheckoutSessionLineItems::default();
    let mut price_data = CreateCheckoutSessionLineItemsPriceData::default();
    let mut product_data = CreateCheckoutSessionLineItemsPriceDataProductData::default();
    product_data.name = "Realy cool course".to_owned();
    price_data.currency = Currency::USD;
    price_data.product_data = Some(product_data);
    price_data.unit_amount = Some(99);
    line_item.price_data = Some(price_data);
    line_item.quantity = Some(1);
    checkout_session_params.line_items = Some(vec![line_item]);
    checkout_session_params.mode = Some(CheckoutSessionMode::Payment);
    let checkout_session = match CheckoutSession::create(&client, checkout_session_params).await {
        Ok(checkout_session) => checkout_session,
        Err(error) => {
            dbg!(error);
            return Redirect::to(&data.cancel_uri);
        }
    };
    let redirect_uri = checkout_session.url.unwrap();
    Redirect::to(&redirect_uri)
}

pub async fn stripe_webhook(data: Bytes, headers: HeaderMap) -> impl IntoResponse {
    if let Some(stripe_signature_header) = headers.get("stripe-signature") {
        let stripe_signature_secret = dotenv!("STRIPE_SIGNATURE");
        let body = String::from_utf8(data.to_vec()).unwrap();
        dbg!(&body);
        Webhook::construct_event(
            &body,
            stripe_signature_header.to_str().unwrap(),
            &stripe_signature_secret,
        )
        .unwrap();

        dbg!("here?");
    }
    StatusCode::OK
}

#[derive(Deserialize, Serialize, Debug)]
pub struct HandleStripePaymentBody {
    pub cancel_uri: String,
    pub success_uri: String,
}
