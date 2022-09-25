//! This library is extracted from a personal project, 
//! and is currently only used for a Telegram bot

use serde_json::{Value, from_value, Error};
use structs::DFQueryWrapper;

pub mod structs;
pub mod responses;

/// Try to parse the webhook request data and returns a struct representation
pub fn parse_dialogflow_webhook_data(value: Value) -> Result<DFQueryWrapper, Error> {
    from_value(value)
}