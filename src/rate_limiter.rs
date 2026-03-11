use std::{num::NonZeroU32, sync::Arc};

use governor::{DefaultDirectRateLimiter, Quota, RateLimiter};

pub enum RateLimitPreset {
    Delivery,
    Management,
    Custom(NonZeroU32),
}

impl RateLimitPreset {
    fn per_second(&self) -> NonZeroU32 {
        match self {
            Self::Delivery => Option::expect(NonZeroU32::new(100), "Provided value cannot be used"),
            Self::Management => {
                Option::expect(NonZeroU32::new(10), "Provided value cannot be used")
            }
            Self::Custom(n) => *n,
        }
    }
}

pub struct ClientRateLimiter {
    inner: DefaultDirectRateLimiter,
}

impl ClientRateLimiter {
    pub fn new(preset: RateLimitPreset) -> Arc<Self> {
        let quota = Quota::per_second(preset.per_second());

        Arc::new(Self {
            inner: RateLimiter::direct(quota),
        })
    }

    pub async fn until_ready(&self) {
        self.inner.until_ready().await
    }
}
