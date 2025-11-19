#[derive(Clone)]
pub struct User {
    pub(crate) name: String,
    pub(crate) lang: String,
    pub(crate) token: String,
    pub(crate) expires: u64,
}
