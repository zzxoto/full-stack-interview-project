use std::{collections::HashSet, sync::Arc};

use crate::cardano::api::CardanoApi;

use super::book::{Book, BookId};

pub struct Bookshelf {
    api: Arc<Box<dyn CardanoApi>>,
    stake_address: String,
}

impl Bookshelf {
    pub fn new(api: Arc<Box<dyn CardanoApi>>, stake_address: String) -> Self {
        Bookshelf { api, stake_address }
    }

    pub async fn get_books(&self, policy_ids: HashSet<String>) -> anyhow::Result<Vec<Book>> {
        todo!()
    }

    pub async fn has_book(&self, id: &BookId) -> bool {
        todo!()
    }
}
