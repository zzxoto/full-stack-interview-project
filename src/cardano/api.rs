use super::model::{Asset, GetAddressSummary};

#[async_trait::async_trait]
pub trait CardanoApi: Send + Sync + 'static {
    async fn get_address_summary(&self, address: &str) -> anyhow::Result<GetAddressSummary>;
    async fn get_all_addresses(&self, stake_address: &str) -> anyhow::Result<Vec<String>>;
    async fn get_address_assets(&self, address: &str) -> anyhow::Result<Vec<Asset>>;
    async fn get_asset_721_metadata(
        &self,
        asset_id: &str,
    ) -> anyhow::Result<Option<serde_json::Value>>;
}
