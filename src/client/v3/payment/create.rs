use std::future::Future;
use truelayer_rust::{TrueLayerClient, apis::auth::Credentials, Error};
use truelayer_rust::apis::payments::{AccountIdentifier, Currency, Beneficiary, CreatePaymentRequest, CreatePaymentUserRequest, PaymentMethod, ProviderSelection, CreatePaymentResponse};
use anyhow::Result;

pub async fn create_payment(client: &truelayer_rust::TrueLayerClient) -> anyhow::Result<String> {
    let payment_id = match client.payments.create(&CreatePaymentRequest{
        amount_in_minor: 0,
        currency: Currency::Gbp,
        payment_method: PaymentMethod::BankTransfer {
            provider_selection: ProviderSelection::Preselected {
                provider_id: "".to_string(),
                scheme_id: "".to_string(),
                remitter: None
            },
            beneficiary: Beneficiary::ExternalAccount {
                account_holder_name: "john doe".to_string(),
                account_identifier: AccountIdentifier::SortCodeAccountNumber {
                    sort_code: "000000".to_string(),
                    account_number:  "12345678".to_string()
                },
                reference: "".to_string()
            }
        },
        user: CreatePaymentUserRequest::NewUser{
            name: Some("john doe".to_string()),
            email: Some("a@a.com".to_string()),
            phone: None
        },
        metadata: None
    }).await {
        Ok(response) => {
            response.id
        }
        Err(e) => panic!("{:?}", e)
    };

    Ok(payment_id)
}