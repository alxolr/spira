use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::Response;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct IncidentStatusDto {
    #[serde(rename = "IncidentStatusId")]
    pub incident_status_id: Option<u64>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Active")]
    pub active: Option<bool>,
    #[serde(rename = "Open")]
    pub open: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ProjectTemplateDto {
    #[serde(rename = "ProjectTemplateId")]
    pub project_template_id: Option<u64>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Active")]
    pub description: Option<String>,
    #[serde(rename = "IsActive")]
    pub is_active: Option<bool>,
}

pub struct ProjectTemplateClient<'a> {
    client: Client,
    base_url: &'a str,
}

impl<'a> ProjectTemplateClient<'a> {
    pub fn new(client: Client, base_url: &'a str) -> Self {
        ProjectTemplateClient { client, base_url }
    }

    pub async fn list(&self) -> Response<Vec<ProjectTemplateDto>> {
        let projects = self
            .client
            .get(self.append_to_url("/project-templates"))
            .send()
            .await?;

        println!("{:#?}", projects);
            // .json::<Vec<ProjectTemplateDto>>()
            // .await?;

        Ok(vec![])
    }

    pub async fn incident_status_list(
        &self,
        project_template_id: u64,
    ) -> Response<Vec<IncidentStatusDto>> {
        let incident_statuses = self
            .client
            .get(self.append_to_url(&format!(
                "/project-templates/{}/incidents/statuses",
                project_template_id
            )))
            .send()
            .await?
            .json::<Vec<IncidentStatusDto>>()
            .await?;

        Ok(incident_statuses)
    }

    fn append_to_url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }
}
