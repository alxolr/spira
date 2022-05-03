use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::Response;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ProjectDto {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "ProjectId")]
    pub project_id: Option<u64>,
}

pub struct ProjectClient<'a> {
    client: Client,
    base_url: &'a str,
}

impl<'a> ProjectClient<'a> {
    pub fn new(client: Client, base_url: &'a str) -> Self {
        ProjectClient { client, base_url }
    }

    pub async fn list(&self) -> Response<Vec<ProjectDto>> {
        let projects = self
            .client
            .get(self.append_to_url("/projects"))
            .send()
            .await?
            .json::<Vec<ProjectDto>>()
            .await?;

        Ok(projects)
    }

    fn append_to_url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }
}
