use chrono::{DateTime, Utc};

/// A monitoring sample
pub struct Sample {
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
