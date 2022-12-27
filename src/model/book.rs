use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct BookId(pub String);

impl Display for BookId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BookId({})", &self.0)
    }
}

#[derive(Clone, Debug)]
pub struct Book {
    pub id: BookId,
    pub title: String,
    pub image_url: String,
}
