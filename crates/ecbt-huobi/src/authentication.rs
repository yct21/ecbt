pub(crate) struct Credentials {
    /// used in API request
    api_key: String,
    /// to generate the signature
    api_secret: String,
}

pub(crate) enum Auth {
    WithCredentials(Credentials),
    WithoutCredentials,
}
