/*
 * OpenTTD Content API
 *
 * OpenTTD Content API
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserGet200Response {
    #[serde(rename = "display-name", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

impl UserGet200Response {
    pub fn new() -> UserGet200Response {
        UserGet200Response {
            display_name: None,
        }
    }
}

