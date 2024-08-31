use std::{
    pin::Pin,
    task::{Context, Poll},
    time::Instant,
};

use tonic::{body::BoxBody, Status};
use tower::{Layer, Service};

#[derive(Debug, Clone, Default)]
pub struct LoggerMiddlewareLayer {}

impl<S> Layer<S> for LoggerMiddlewareLayer {
    type Service = LoggerMiddleware<S>;

    fn layer(&self, service: S) -> Self::Service {
        LoggerMiddleware { inner: service }
    }
}

#[derive(Debug, Clone)]
pub struct LoggerMiddleware<S> {
    inner: S,
}

type BoxFuture<'a, T> = Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>;

impl<S> Service<hyper::Request<BoxBody>> for LoggerMiddleware<S>
where
    S: Service<hyper::Request<BoxBody>, Response = hyper::Response<BoxBody>>
        + Clone
        + Send
        + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: hyper::Request<BoxBody>) -> Self::Future {
        let start = Instant::now();

        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);

        Box::pin(async move {
            let uri = req
                .uri()
                .to_string()
                .replace("http://", "")
                .replace("https://", "");
            let version = req.version();
            let content_type = req
                .headers()
                .get("content-type")
                .ok_or(Status::invalid_argument("Missing content-type Header"))
                .map_err(|err| err)
                .unwrap()
                .to_str()
                .map_err(|_| Status::invalid_argument("Missing content-type Header"))
                .unwrap()
                .to_owned();

            // Do extra async work here...
            let response = inner.call(req).await?;

            log::info!(
                "uri={:?}; version={:?}; content_type={:?} time_taken={:?}",
                uri,
                version,
                content_type,
                start.elapsed()
            );
            Ok(response)
        })
    }
}
