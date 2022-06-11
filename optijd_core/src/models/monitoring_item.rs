use crate::prelude::*;

pub struct MonitoringItem {
    /// The unique identifier of this record
    id: u32,

    /// a proper convenient name to give to your monitoring instance.
    name: String,

    /// The protocol to use for this request (defaults to HTTP_HTTPS)
    protocol: Protocol,

    /// The url to check for
    url: String,

    /// The HTTP method to use for checking the given 'url'
    method: super::http_method::HttpMethod,

    /// Heartbeat Interval (Check every 60 seconds)
    heartbeat_in_seconds: u32,

    /// Maximum retries before the service is marked as down and a notification is sent
    retries: u8,

    /// Option to ignore tls or ssl errors for HTTPS websites.
    ignore_tls_ssl_errors: bool,

    /// Select status codes which are considered as a successful response.
    accepted_status_codes: Vec<StatusCodeRange>,

    /// Optional tags to assign to this entry
    tags: Vec<String>,
}
