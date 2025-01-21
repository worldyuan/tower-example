use std::{future::Future, pin::Pin, sync::{atomic::AtomicUsize, Arc}, task::Poll};

#[derive(Default)]
pub struct DemoApp {
    counter: Arc<AtomicUsize>,
}

impl tower::Service<crate::http::Request> for DemoApp {
    type Response = crate::http::Response;

    type Error = anyhow::Error;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: crate::http::Request) -> Self::Future {
        let counter = self.counter.clone();
        Box::pin(async move {
            println!("Handling a request for {}", req.path_and_query);
            let counter = counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            anyhow::ensure!(counter % 4 != 2, "Failing 25% of the time, just for fun");
            req.headers.insert("X-Counter".to_owned(), counter.to_string());
            let res = crate::http::Response {
                status: 200,
                headers: req.headers,
                body: req.body,
            };
            Ok::<_, anyhow::Error>(res)
        })
    }
}
