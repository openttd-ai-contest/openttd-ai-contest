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
pub struct Package {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "content-type")]
    pub content_type: models::ContentType,
    #[serde(rename = "unique-id")]
    pub unique_id: String,
    #[serde(rename = "archived", skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(rename = "replaced-by", skip_serializing_if = "Option::is_none")]
    pub replaced_by: Option<Box<models::PackageAllOfReplacedBy>>,
    #[serde(rename = "authors", skip_serializing_if = "Option::is_none")]
    pub authors: Option<Vec<models::Author>>,
    #[serde(rename = "versions", skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<models::VersionMinimized>>,
}

impl Package {
    pub fn new(content_type: models::ContentType, unique_id: String) -> Package {
        Package {
            name: None,
            description: None,
            url: None,
            tags: None,
            content_type,
            unique_id,
            archived: None,
            replaced_by: None,
            authors: None,
            versions: None,
        }
    }
}
