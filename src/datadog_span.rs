use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub(crate) struct DatadogSpan {
    parent_id: u64,
    trace_id: u64,
    metrics: HashMap<String, serde_json::Value>,
    duration: u64,
    span_id: u64,
    name: String,
    service: String,
    resource: String,
    meta: HashMap<String, String>,
    error: i64,
    start: u64,
}