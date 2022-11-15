use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::Response;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default)]
/// The Release fields
pub struct ReleaseDto {
    /// The id of the incident (integer)
    #[serde(rename = "ReleaseId")]
    pub release_id: Option<u64>,

    /// The id of the incident (integer)
    #[serde(rename = "FullName")]
    pub full_name: Option<String>,
}

/// The Requirement Artifact Submodule
pub struct ReleaseClient<'a> {
    client: Client,
    base_url: &'a str,
}

impl<'a> ReleaseClient<'a> {
    pub fn new(client: Client, base_url: &'a str) -> Self {
        ReleaseClient { client, base_url }
    }

    /// Retrieves all the releases belonging to the current project
    pub async fn list(&self, project_id: u64) -> Response<Vec<ReleaseDto>> {
        let path = &format!("/projects/{}/releases", project_id);

        let releases = self
            .client
            .get(self.append_to_url(path))
            .send()
            .await?
            .json::<Vec<ReleaseDto>>()
            .await?;

        Ok(releases)
    }

    fn append_to_url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }
}
