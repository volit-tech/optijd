use chrono::{DateTime, Utc};

/// The http method types that can be used for uptime monitoring.
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
}

/// The protocol to use for uptime monitoring requests
pub enum Protocol {
    HTTP_HTTPS,
    TCP,
    PING,
    DNS,
    PUSH,
    STEAM_GAME_SERVER,
}

/// A range of status codes defines by a minimum and a maximum value
pub struct StatusCodeRange {
    min: u32,
    max: u32,
}

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
    method: HttpMethod,

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

/// A monitoring sample
pub struct MonitorResult {
    /// The unique identifier of this record
    id: u32,

    /// The unique identifier of the parent monitoring item
    fk_monitoring_item: u32,

    /// The timestamp in UTC on which the sample was taken.
    timestamp: DateTime<Utc>,

    /// The status code that was returned for the sample.
    status_code: u32,

    /// The response message we got from the server for this sample.
    response_message: String,
}
