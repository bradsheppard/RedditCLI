#[derive(Debug)]
pub struct Comment {
    pub body: String,
    pub replies: Vec<Comment>
}
