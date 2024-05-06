/**
 * Author: ds
 * Date: 2021
 * A basic rust web server's middleware.
 */
mod middleware {
    use hyper::{Body, Request, Response, Next};
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll};

    pub struct LoggingMiddleware<S> {
        inner: S,
    }

    impl<S> LoggingMiddleware<S> {
        pub fn new(inner:S) -> LoggingMiddleware<S> {
            LoggingMiddleware { inner }
        }
    }

    impl <S> hyper::service::Service<Request<Body>> for LoggingMiddleware<S>
    where
        S: hyper::service::Service<Request<Body>, Response = Response<Body>> + Clone + Send + 'static,
        S::Future: Send + 'static,
    {
        type Response = S::Response;
        type Error = S::Error;
        type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

        fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            self.innter.poll_ready(cx)
        }

        fn call(&mut self, req: Request<Body>) -> Self::Future {
            let fut = self.inner.call(req);
            Box::pin(async move {
                let res = fut.await?;

                //Log the req and res here
                Ok(res)
            })
        }
    } 
}