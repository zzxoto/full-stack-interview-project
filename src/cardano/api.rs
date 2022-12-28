use std::collections::HashSet;

use super::model::Asset;

#[async_trait::async_trait]
pub trait CardanoApi: Send + Sync + 'static {
    /**
     * Given a stake address, returns all known associated addresses.
     */
    async fn get_all_addresses(&self, stake_address: &str) -> anyhow::Result<Vec<String>>;

    /**
     * Given an address, returns all assets "owned" by that address.
     */
    async fn get_address_assets(&self, address: &str) -> anyhow::Result<Vec<Asset>>;

    /**
     * Given an asset id (concat of policy_id and hex encoded asset_name) returns a
     * set of addresses that own the asset.
     */
    async fn get_asset_addresses(&self, asset_id: &str) -> anyhow::Result<HashSet<String>>;
}
