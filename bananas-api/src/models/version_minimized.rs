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
pub struct VersionMinimized {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "license", skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(rename = "upload-date")]
    pub upload_date: String,
    #[serde(rename = "md5sum-partial", skip_serializing_if = "Option::is_none")]
    pub md5sum_partial: Option<String>,
    #[serde(rename = "filesize", skip_serializing_if = "Option::is_none")]
    pub filesize: Option<i32>,
    #[serde(rename = "availability", skip_serializing_if = "Option::is_none")]
    pub availability: Option<Availability>,
    #[serde(rename = "dependencies", skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<models::Dependency>>,
    #[serde(rename = "compatibility", skip_serializing_if = "Option::is_none")]
    pub compatibility: Option<Vec<models::Compatibility>>,
}

impl VersionMinimized {
    pub fn new(upload_date: String) -> VersionMinimized {
        VersionMinimized {
            name: None,
            description: None,
            url: None,
            tags: None,
            version: None,
            license: None,
            upload_date,
            md5sum_partial: None,
            filesize: None,
            availability: None,
            dependencies: None,
            compatibility: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Availability {
    #[serde(rename = "savegames-only")]
    SavegamesOnly,
    #[serde(rename = "new-games")]
    NewGames,
}

impl Default for Availability {
    fn default() -> Availability {
        Self::SavegamesOnly
    }
}

