use std::{future::Future, task::Poll};

pub struct AppFn<F> {
    f: F,
}

pub fn app_fn<F, Ret>(f: F) -> AppFn<F>
where
    F: FnMut(crate::http::Request) -> Ret,
    Ret: Future<Output = Result<crate::http::Response, anyhow::Error>>,
{
    AppFn { f }
}

impl<F, Ret> tower::Service<crate::http::Request> for AppFn<F>
where
    F: FnMut(crate::http::Request) -> Ret,
    Ret: Future<Output = Result<crate::http::Response, anyhow::Error>>,
{
    type Response = crate::http::Response;

    type Error = anyhow::Error;

    type Future = Ret;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: crate::http::Request) -> Self::Future {
        (self.f)(req)
    }
}
