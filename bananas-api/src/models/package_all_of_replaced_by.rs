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
pub struct PackageAllOfReplacedBy {
    #[serde(rename = "unique-id", skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
}

impl PackageAllOfReplacedBy {
    pub fn new() -> PackageAllOfReplacedBy {
        PackageAllOfReplacedBy {
            unique_id: None,
        }
    }
}

