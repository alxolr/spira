use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::Response;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default)]
/// The Incident fields
pub struct IncidentDto {
    /// The id of the incident (integer)
    #[serde(rename = "IncidentId")]
    pub incident_id: Option<u64>,

    /// The id of the priority of the incident (integer)
    #[serde(rename = "PriorityId")]
    pub priority_id: Option<u64>,

    /// The id of the severity of the incident (integer)
    #[serde(rename = "SeverityId")]
    pub severity_id: Option<u64>,

    /// The id of the status of the incident (integer) If no value is provided,
    /// the default status for the workflow is used
    #[serde(rename = "IncidentStatusId")]
    pub incident_status_id: Option<u64>,

    /// The id of the type of the incident (integer) If no value is provided,
    /// the default type for the project is used
    #[serde(rename = "IncidentTypeId")]
    pub incident_type_id: Option<u64>,

    /// The id of the user who detected the incident (integer) If a value is not provided,
    /// the authenticated user is used
    #[serde(rename = "OpenerId")]
    pub opener_id: Option<u64>,

    /// The id of the user that the requirement is assigned-to (integer)
    #[serde(rename = "OwnerId")]
    pub owner_id: Option<u64>,

    /// The id of the test run steps that the incident relates to (integer)
    #[serde(rename = "TestRunStepIds")]
    pub test_run_step_ids: Option<u64>,

    /// The id of the release/iteration that the incident was detected in (integer)
    #[serde(rename = "DetectedReleaseId")]
    pub detected_release_id: Option<u64>,

    /// The id of the release/iteration that the incident will be fixed in (integer)
    #[serde(rename = "ResolvedReleaseId")]
    pub resolved_release_id: Option<u64>,

    /// The id of the release/iteration that the incident was retested in (integer)
    #[serde(rename = "VerifiedReleaseId")]
    pub verified_release_id: Option<u64>,

    /// The list of components that this incident belongs to (array of integers)
    #[serde(rename = "ComponentIds")]
    pub component_ids: Option<Vec<u64>>,

    /// The name of the incident (string)
    #[serde(rename = "Name")]
    pub name: String,

    /// The description of the incident (string)
    #[serde(rename = "Description")]
    pub description: Option<String>,

    /// The date/time that the incident was originally created If no value is provided,
    /// the current date/time on the server is used (date-time)
    /// { "CreationDate": "2022-03-14T11:28:07.240Z" },
    #[serde(rename = "CreationDate")]
    pub creation_date: Option<String>,

    /// The date that work started on the incident (date-time)
    #[serde(rename = "StartDate")]
    pub start_date: Option<String>,

    /// The date that work is scheduled to finish on the incident (date-time)
    #[serde(rename = "EndDate")]
    pub end_date: Option<String>,

    /// The date that the incident was closed (date-time)
    #[serde(rename = "ClosedDate")]
    pub closed_date: Option<String>,

    /// The estimated effort (in minutes) to resolve the incident (integer)
    #[serde(rename = "EstimatedEffort")]
    pub estimated_effort: Option<u64>,

    /// The actual effort (in minutes) it took to resolve the incident (integer)
    #[serde(rename = "ActualEffort")]
    pub actual_effort: Option<u64>,

    /// The effort remaining as reported by the developer
    #[serde(rename = "RemainingEffort")]
    pub remaining_effort: Option<u64>,

    /// The date/time that the task was last modified This field needs to match the values retrieved to ensure data-concurrency
    #[serde(rename = "LastUpdateDate")]
    pub last_update_date: Option<String>,

    /// The id of the build that the incident was fixed in (integer)
    #[serde(rename = "FixedBuildId")]
    pub fixed_build_id: Option<u64>,

    /// The id of the build that the incident was detected in (integer)
    #[serde(rename = "DetectedBuildId")]
    pub detected_build_id: Option<u64>,

    /// The id of the project that the artifact belongs to
    #[serde(rename = "ProjectId")]
    pub project_id: u64,

    /// The datetime used to track optimistic concurrency to prevent edit conflicts
    #[serde(rename = "ConcurrencyDate")]
    pub concurrency_date: Option<String>,

    /// Does this artifact have any attachments?
    #[serde(rename = "IsAttachments")]
    pub is_attachements: Option<bool>,
}

/// The Requirement Artifact Submodule
pub struct IncidentClient<'a> {
    client: Client,
    base_url: &'a str,
}

impl<'a> IncidentClient<'a> {
    pub fn new(client: Client, base_url: &'a str) -> Self {
        IncidentClient { client, base_url }
    }

    /// Retrieves all incidents owned by the currently authenticated user
    pub async fn list_my(&self) -> Response<Vec<IncidentDto>> {
        let incidents = self
            .client
            .get(self.append_to_url("/incidents"))
            .send()
            .await?
            .json::<Vec<IncidentDto>>()
            .await?;

        Ok(incidents)
    }

    /// Retrieves a single incident in the system
    pub async fn get(&self, project_id: u64, incident_id: u64) -> Response<IncidentDto> {
        let path = &format!("/projects/{}/incidents/{}", project_id, incident_id);

        let incident = self
            .client
            .get(self.append_to_url(path))
            .send()
            .await?
            .json::<IncidentDto>()
            .await?;

        Ok(incident)
    }

    /// Creates a new incident in the specified project in the system
    pub async fn create(&self, project_id: u64, incident: IncidentDto) -> Response<IncidentDto> {
        let json_incident = serde_json::to_string(&incident)?;
        let incident = self
            .client
            .post(self.append_to_url(&format!("/projects/{}/incidents", project_id)))
            .body(json_incident)
            .send()
            .await?
            .json::<IncidentDto>()
            .await?;

        Ok(incident)
    }

    /// Updates an incident in the system
    pub async fn update(&self, project_id: u64, incident: IncidentDto) -> Response<()> {
        let json_incident = serde_json::to_string(&incident)?;
        self.client
            .put(self.append_to_url(&format!(
                "/projects/{}/incidents/{}",
                project_id,
                incident.incident_id.unwrap()
            )))
            .body(json_incident)
            .send()
            .await?;

        Ok(())
    }

    /// Deletes an incident in the system
    pub async fn delete(&self, project_id: u64, incident_id: u64) -> Response<()> {
        self.client
            .delete(self.append_to_url(&format!(
                "/projects/{}/incidents/{}",
                project_id, incident_id
            )))
            .send()
            .await?;

        Ok(())
    }

    fn append_to_url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }
}
