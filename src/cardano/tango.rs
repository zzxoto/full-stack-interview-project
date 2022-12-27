use super::{
    api::CardanoApi,
    model::{Asset, GetAddressSummary},
};

pub struct Tango {
    base_url: String,
    app_id: String,
    api_key: String,
}

impl Tango {
    pub fn new(base_url: String, app_id: String, api_key: String) -> anyhow::Result<Self> {
        Ok(Tango {
            base_url,
            app_id,
            api_key,
        })
    }
}

#[async_trait::async_trait]
impl CardanoApi for Tango {
    async fn get_address_summary(&self, address: &str) -> anyhow::Result<GetAddressSummary> {
        todo!()
    }

    async fn get_all_addresses(&self, stake_address: &str) -> anyhow::Result<Vec<String>> {
        todo!()
    }

    async fn get_address_assets(&self, address: &str) -> anyhow::Result<Vec<Asset>> {
        todo!()
    }

    async fn get_asset_721_metadata(
        &self,
        asset_id: &str,
    ) -> anyhow::Result<Option<serde_json::Value>> {
        todo!()
    }
}
