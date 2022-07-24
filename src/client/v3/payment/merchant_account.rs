use anyhow::Error;
use truelayer_rust::apis::payments::Currency;

pub async fn get_gbp_merchant_account_id(
    client: &truelayer_rust::TrueLayerClient,
) -> anyhow::Result<String> {
    client
        .merchant_accounts
        .list()
        .await?
        .iter()
        .filter(|ma| ma.currency == Currency::Gbp)
        .map(|ma| ma.id.clone())
        .last()
        .ok_or_else(|| Error::msg("No GBP merchant account found"))
}
