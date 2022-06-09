pub(super) struct Transport {
    url: String,
}

impl Transport {
    pub(crate) fn new(url: String) -> Self {
        Self { url }
    }
}
