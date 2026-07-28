use crate::model::Report;

/// Serializes a report as pretty-printed JSON.
///
/// # Errors
///
/// Returns an error if the report cannot be serialized as JSON.
pub fn render(report: &Report) -> serde_json::Result<String> {
    serde_json::to_string_pretty(report)
}
