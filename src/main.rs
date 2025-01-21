use std::{
    collections::HashMap,
    future::Future,
    pin::Pin,
    sync::{atomic::AtomicUsize, Arc},
    task::Poll,
};
mod app;
mod http;
mod util;

#[tokio::main]
async fn main() {
    let counter = Arc::new(AtomicUsize::new(0));
    /* 错误
    let counter = Arc::clone(&counter);
    util::app_fn(move |mut req| {

        let counter = counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Err(anyhow::anyhow!("1"))
    });
    */

    util::app_fn(move |mut req| {
        let counter = counter.clone();
        async move {
            let counter = counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            req.headers
                .insert("X-Counter".to_owned(), counter.to_string());
            let res = crate::http::Response {
                status: 200,
                headers: req.headers,
                body: req.body,
            };
            Ok::<_, anyhow::Error>(res)
        }
    });
}
