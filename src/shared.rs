use serde::Deserialize;
use std::collections::HashMap;

pub type PageId = String;

#[derive(Deserialize, Debug)]
pub struct Story {
    pub pages: HashMap<PageId, Page>,
}

#[derive(Deserialize, Debug)]
pub struct Page {
    pub content: String,
    pub choices: Option<Vec<Choice>>,
}

#[derive(Deserialize, Debug)]
pub struct Choice {
    pub text: String,
    pub to: PageId,
}
