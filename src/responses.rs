
use serde_json::{json, Value};

/// Build a response compliant with DialogFlow webhook response
/// Accepts a vec of strings as responses to send
pub fn build_response(responses: Vec<String>) -> Value {
    json!(
        {
            "fulfillmentText": responses.join("\n"),
            "fulfillmentMessages": [
                {
                    "text": {
                        "text": vec![responses.join("\n")],
                    }
                }
            ]
        }
    )
}
