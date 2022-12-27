pub struct GetAddressSummary {
    pub network: String,
    pub address: String,
    pub stake_address: Option<String>,
}

pub struct Asset {
    pub policy_id: String,
    pub asset_name: String,
    pub quantity: i64,
}
