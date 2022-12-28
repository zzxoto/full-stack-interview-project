use std::{collections::HashSet, sync::Arc};

use crate::cardano::api::CardanoApi;

use super::book::{BookId, BookListItem};

pub struct Bookshelf {
    api: Arc<Box<dyn CardanoApi>>,
    stake_address: String,
}

impl Bookshelf {
    pub fn new(api: Arc<Box<dyn CardanoApi>>, stake_address: String) -> Self {
        Bookshelf { api, stake_address }
    }

    /**
     * Gets the collection of books available on this bookshelf.
     */
    pub async fn get_books(
        &self,
        policy_ids: HashSet<String>,
    ) -> anyhow::Result<Vec<BookListItem>> {
        todo!()
    }

    /**
     * Returns true if the book exists on the bookshelf and false otherwise.
     */
    pub async fn has_book(&self, id: &BookId) -> bool {
        // bonus points if you can implement this more efficiently than just
        // calling get_books and seeing if the BookId exists in that set.
        todo!()
    }
}
