use std::{collections::HashMap, future::Future, task::Poll};

pub struct Request {
    pub path_and_query: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

pub struct Response {
    pub status: u32,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

// 另外一种方案
