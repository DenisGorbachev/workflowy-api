#[derive(Default, Eq, PartialEq, Clone, Copy, Debug)]
pub struct Limiter;

pub type WorkflowyRateLimiter = governor::DefaultDirectRateLimiter;
