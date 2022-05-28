use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ListingResponse {
    pub data: ListingData
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListingData {
    pub children: Vec<Comment>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Comment {
    pub data: CommentData 
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommentData {
    pub body: Option<String>
}
