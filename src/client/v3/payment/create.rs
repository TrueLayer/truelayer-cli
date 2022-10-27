use crate::client::v3::payment::merchant_account::get_gbp_merchant_account_id;
use truelayer_rust::apis::payments::{AccountIdentifier, Beneficiary, CreatePaymentRequest, CreatePaymentUserRequest, Currency, PaymentMethod, PaymentMethodRequest, ProviderSelection, ProviderSelectionRequest};

pub async fn create_external_account_payment(
    client: &truelayer_rust::TrueLayerClient,
) -> anyhow::Result<String> {
    let resp = client
        .payments
        .create(&CreatePaymentRequest {
            amount_in_minor: 15,
            currency: Currency::Gbp,
            payment_method: PaymentMethodRequest::BankTransfer {
                provider_selection: ProviderSelectionRequest::Preselected {
                    provider_id: "mock-payments-gb-redirect".to_string(),
                    scheme_id: "faster_payments_service".to_string(),
                    remitter: None,
                },
                beneficiary: Beneficiary::ExternalAccount {
                    account_holder_name: "John doe".to_string(),
                    account_identifier: AccountIdentifier::SortCodeAccountNumber {
                        sort_code: "000000".to_string(),
                        account_number: "12345678".to_string(),
                    },
                    reference: "(LegacyReturn)".to_string(),
                },
            },
            user: CreatePaymentUserRequest::NewUser {
                name: Some("john doe".to_string()),
                email: Some("a@a.com".to_string()),
                phone: None,
            },
            metadata: None,
        })
        .await?;
    Ok(resp.id)
}
