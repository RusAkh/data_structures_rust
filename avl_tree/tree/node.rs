#[derive(Debug)]
pub struct Node {
    pub right: Option<*mut Node>,
    pub left: Option<*mut Node>,
    pub value: i64,
    pub height: i64,
    pub balance: i64,
}
