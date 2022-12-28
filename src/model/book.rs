use std::fmt::Display;

/**
 * Represents a unique identifier for a book.
 */
#[derive(Clone, Debug)]
pub struct BookId(pub String);

impl BookId {
    pub fn new(policy_id: String, asset_name: String) -> Self {
        BookId(format!("{}{}", policy_id, asset_name))
    }

    /**
     * Gets an asset_id from the BookId.
     */
    pub fn as_asset_id(&self) -> String {
        self.0.clone()
    }
}

impl Display for BookId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BookId({})", &self.0)
    }
}

#[derive(Clone, Debug)]
pub struct BookListItem {
    pub id: BookId,
    // use the non-hex-encoded "asset_name" for this field
    pub token_name: String,
}
