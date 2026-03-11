use std::sync::Arc;

use async_trait::async_trait;
use http::Extensions;
use reqwest::{Request, Response};
use reqwest_middleware::{Middleware, Next};

use crate::rate_limiter::ClientRateLimiter;

pub struct RateLimiterMiddleware {
    pub rate_limiter: Arc<ClientRateLimiter>,
}

#[async_trait]
impl Middleware for RateLimiterMiddleware {
    async fn handle(
        &self,
        req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> reqwest_middleware::Result<Response> {
        self.rate_limiter.until_ready().await;
        next.run(req, extensions).await
    }
}
