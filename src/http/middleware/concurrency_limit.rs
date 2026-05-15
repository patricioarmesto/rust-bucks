use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll};

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use tower::{Layer, Service};

#[derive(Clone)]
pub struct ConcurrencyLimitLayer {
    max: usize,
}

impl ConcurrencyLimitLayer {
    pub fn new(max: usize) -> Self {
        Self { max }
    }
}

impl<S> Layer<S> for ConcurrencyLimitLayer {
    type Service = ConcurrencyLimitService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        ConcurrencyLimitService {
            inner,
            current: Arc::new(AtomicUsize::new(0)),
            max: self.max,
        }
    }
}

#[derive(Clone)]
pub struct ConcurrencyLimitService<S> {
    inner: S,
    current: Arc<AtomicUsize>,
    max: usize,
}

impl<S> Service<axum::extract::Request> for ConcurrencyLimitService<S>
where
    S: Service<axum::extract::Request, Response = Response>,
    S::Future: Send + 'static,
{
    type Response = Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: axum::extract::Request) -> Self::Future {
        let previous = self.current.fetch_add(1, Ordering::AcqRel);

        if previous >= self.max {
            self.current.fetch_sub(1, Ordering::AcqRel);
            return Box::pin(async move {
                Ok((StatusCode::SERVICE_UNAVAILABLE, "too many requests").into_response())
            });
        }

        let current = Arc::clone(&self.current);
        let future = self.inner.call(req);

        Box::pin(async move {
            let response = future.await?;
            current.fetch_sub(1, Ordering::AcqRel);
            Ok(response)
        })
    }
}
