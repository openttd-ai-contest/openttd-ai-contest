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
pub struct ConfigLicensesGet200ResponseInner {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "deprecated", skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
}

impl ConfigLicensesGet200ResponseInner {
    pub fn new() -> ConfigLicensesGet200ResponseInner {
        ConfigLicensesGet200ResponseInner {
            name: None,
            deprecated: None,
        }
    }
}

