pub trait CustomRequestBuilder {
    fn apply_default_header(self) -> Self;
    fn add_authorization(self, access_token: String) -> Self;
}

impl CustomRequestBuilder for reqwest::RequestBuilder {
    fn apply_default_header(self) -> Self {
        self.header("Content-Type", "application/json")
            .header("X-Correlation-Id", "and-7-86-2-1000005524.9twu3pgr")
            .header("User-Agent", "Cashtools Client - cl3t0")
    }
    fn add_authorization(self, access_token: String) -> Self {
        self.header("Authorization", format!("Bearer {}", access_token))
    }
}
