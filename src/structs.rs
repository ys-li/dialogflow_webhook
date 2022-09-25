use serde_json::Value;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct DFQueryIntent {
    display_name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DFQueryWrapper {
    pub query_result: DFQueryResult,
    pub original_detect_intent_request: DFOriginalDetectIntentRequest,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DFOriginalDetectIntentRequest {
    // Telegram
    pub payload: DFOriginalDetectIntentRequestPayload,
}

#[derive(Deserialize)]
pub struct DFOriginalDetectIntentRequestPayload {
    pub data: DFOriginalDetectIntentRequestPayloadData,
}

#[derive(Deserialize)]
pub struct DFOriginalDetectIntentRequestPayloadData {
    pub chat: DFOriginalDetectIntentRequestPayloadDataChat,
}

#[derive(Deserialize)]
pub struct DFOriginalDetectIntentRequestPayloadDataChat {
    pub id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DFQueryResult {
    pub output_contexts: Vec<DFOutputContext>,
    #[serde(rename = "intent")]
    _intent: DFQueryIntent,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DFOutputContext {
    pub parameters: Value,
}

impl DFQueryResult {
    /// Intent as a string from this query
    pub fn intent(&self) -> &str {
        self._intent.display_name.as_str()
    }

    /// Parameters of this query
    pub fn parameters(&self) -> Value {
        self.output_contexts[0].parameters.clone()
    }
}
