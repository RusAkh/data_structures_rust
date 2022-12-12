
pub struct Node {
    pub right: Option<*mut Node>,
    pub left: Option<*mut Node>,
    pub value: Option<i64>,
    pub height: Option<i64>,
    pub balance: Option<i64>,
}
