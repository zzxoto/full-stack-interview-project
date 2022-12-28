use std::{collections::HashSet, future::Future};

use serde::de::DeserializeOwned;

use crate::cardano::{api::CardanoApi, model::Asset};

use super::model::ApiListRes;

// feel free to add/remove properties as needed.
pub struct TangoClient {
    base_url: String,
    app_id: String,
    api_key: String,
}

impl TangoClient {
    pub fn new(base_url: String, app_id: String, api_key: String) -> anyhow::Result<Self> {
        Ok(TangoClient {
            base_url,
            app_id,
            api_key,
        })
    }
}

/**
 * A helper method that abstracts iterating over an API response
 * that returns a cursor when there are more results available.
 *
 * Example:
 *
 * async fn get_the_things(id: &str, cursor: Option<String>) -> anyhow::Result<ApiListRes<TheThing>>;
 *
 * let id = "something";
 * let res = get_all(|cursor| get_the_things(&id, cursor)).await?;
 */
pub async fn get_all<F, Fut, T>(f: F) -> anyhow::Result<Vec<T>>
where
    F: Fn(Option<String>) -> Fut,
    Fut: Future<Output = anyhow::Result<ApiListRes<T>>>,
    T: DeserializeOwned,
    T: Clone,
{
    let mut data = Vec::new();
    let mut cursor = None;
    loop {
        let res = f(cursor.clone()).await?;

        data.append(&mut res.data.clone());
        cursor = res.cursor;

        match cursor {
            None => break,
            _ => {}
        }
    }

    Ok(data)
}

#[async_trait::async_trait]
impl CardanoApi for TangoClient {
    // recommended api:
    // https://www.tangocrypto.com/api-reference/#/operations/list-stake_address-addresses
    async fn get_all_addresses(&self, stake_address: &str) -> anyhow::Result<Vec<String>> {
        todo!()
    }

    // recommended api:
    // https://www.tangocrypto.com/api-reference/#/operations/list-address-assets
    async fn get_address_assets(&self, address: &str) -> anyhow::Result<Vec<Asset>> {
        todo!()
    }

    // recommended api:
    // https://www.tangocrypto.com/api-reference/#/operations/list-asset-addresses
    async fn get_asset_addresses(&self, asset_id: &str) -> anyhow::Result<HashSet<String>> {
        todo!()
    }
}
