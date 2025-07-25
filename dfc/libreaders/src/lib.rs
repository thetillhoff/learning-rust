pub trait Reader {
    pub fn next_row(&self) -> Option<Vec<String>>;
}
