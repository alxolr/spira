use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::Response;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
/// The Requirement fields
pub struct RequirementDto {
    /// The id of the requirement (integer)
    #[serde(rename = "RequirementId")]
    pub requirement_id: Option<u64>,

    /// The indentation level of the artifact (string) The system uses a set of
    /// three-letter segments to denote indent (e.g. AAA followed by AAB, etc.)
    #[serde(rename = "IndentLevel")]
    pub indent_level: Option<String>,

    /// The id of the requirement's status (integer). If no value is provided,
    /// the default status is used Relevant values: Accepted 5; Completed 10;
    /// Developed 4; Evaluated 7; In Progress 3; Obsolete 8; Planned 2;
    /// Rejected 6; Requested 1; Tested 9.
    #[serde(rename = "StatusId")]
    pub status_id: Option<u64>,

    /// The type of requirement (integer). Relevant values: Package -1; Need 1;
    /// Feature 2; Use Case 3; User Story 4; Quality 5; Design Element 6 Null
    /// can be passed when created if using the default type
    #[serde(rename = "RequirementTypeId")]
    pub requirement_type_id: Option<u64>,

    /// The id of the user that wrote the requirement (integer)
    /// If no value is provided, the authenticated user is used instead
    #[serde(rename = "AuthorId")]
    pub author_id: Option<u64>,

    /// The id of the user that the requirement is assigned-to (integer)
    #[serde(rename = "OwnerId")]
    pub owner_id: Option<u64>,

    /// The id of the importance of the requirement (integer)
    /// Relevant values: 1 - Critical 1; 2 - High 2; 3 - Medium 3; 4 - Low 4
    #[serde(rename = "ReleaseId")]
    pub importance_id: Option<u64>,

    /// The id of the release the requirement is scheduled to implemented in (integer)
    #[serde(rename = "ReleaseId")]
    pub release_id: Option<u64>,

    /// The id of the component the requirement is a part of
    /// (integer - these are created on a per project user by an administrator)
    #[serde(rename = "ComponentId")]
    pub component_id: Option<u64>,

    /// The name of the requirement (string - required for POST)
    #[serde(rename = "Name")]
    pub name: String,

    /// The description of the requirement (string)
    #[serde(rename = "Description")]
    pub description: Option<String>,

    /// The date/time that the task was originally created
    /// { "CreationDate": "2022-03-14T11:28:07.240Z" },
    #[serde(rename = "CreationDate")]
    pub creation_date: Option<String>,

    /// The date/time that the task was last modified This field needs to match the values retrieved to ensure data-concurrency
    #[serde(rename = "LastUpdateDate")]
    pub last_update_date: Option<String>,

    /// Is this a summary requirement or not (boolean)
    #[serde(rename = "Summary")]
    pub summary: Option<bool>,

    /// The estimate of the requirement (decimal - in story points)
    #[serde(rename = "EstimatePoints")]
    pub estimate_points: Option<f32>,

    /// The list of scenarios steps (array - only available for Use Case requirement types)
    #[serde(rename = "Steps")]
    pub steps: Option<Vec<String>>,

    /// The start date of the requirement for planning purposes
    #[serde(rename = "StartDate")]
    pub start_date: Option<String>,

    /// The end date of the requirement for planning purposes
    #[serde(rename = "EndDate")]
    pub end_date: Option<String>,

    /// The originally estimated effort (in minutes) of the task
    #[serde(rename = "EstimatedEffort")]
    pub estimated_effort: Option<u64>,

    /// The percentage complete of the requirement
    #[serde(rename = "PercentComplete")]
    pub percent_complete: Option<f32>,

    /// The Id of the program theme that the requirement belongs to
    #[serde(rename = "ThemeId")]
    pub theme_id: Option<u64>,

    /// The id of the goal that the requirement belongs to
    #[serde(rename = "GoalId")]
    pub goal_id: Option<u64>,

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
pub struct RequirementClient<'a> {
    client: Client,
    base_url: &'a str,
}

impl<'a> RequirementClient<'a> {
    pub fn new(client: Client, base_url: &'a str) -> Self {
        RequirementClient { client, base_url }
    }

    /// Retrieves all requirements owned by the currently authenticated user
    pub async fn list_my(&self) -> Response<Vec<RequirementDto>> {
        let requirements = self
            .client
            .get(self.append_to_url("/requirements"))
            .send()
            .await?
            .json::<Vec<RequirementDto>>()
            .await?;

        Ok(requirements)
    }

    /// Retrieves a single requirement in the system
    pub async fn get(&self, project_id: u64, requirement_id: u64) -> Response<RequirementDto> {
        let path = &format!("/projects/{}/requirements/{}", project_id, requirement_id);

        let requirement = self
            .client
            .get(self.append_to_url(path))
            .send()
            .await?
            .json::<RequirementDto>()
            .await?;

        Ok(requirement)
    }

    /// Creates a new requirement record in the current project at the end of the list the user has access to.
    /// Note: the indent level is set at the same as the last one in the list the user has access to
    pub async fn create(
        &self,
        project_id: u64,
        requirement: RequirementDto,
    ) -> Response<RequirementDto> {
        let json_requirement = serde_json::to_string(&requirement)?;
        let requirement = self
            .client
            .post(self.append_to_url(&format!("/projects/{}/requirements", project_id)))
            .body(json_requirement)
            .send()
            .await?
            .json::<RequirementDto>()
            .await?;

        Ok(requirement)
    }

    /// Updates a requirement in the system
    pub async fn update(&self, project_id: u64, requirement: RequirementDto) -> Response<()> {
        let json_requirement = serde_json::to_string(&requirement)?;
        self.client
            .put(self.append_to_url(&format!("/projects/{}/requirements", project_id)))
            .body(json_requirement)
            .send()
            .await?;

        Ok(())
    }

    /// Deletes a requirement in the system
    pub async fn delete(&self, project_id: u64, requirement_id: u64) -> Response<()> {
        self.client
            .delete(self.append_to_url(&format!(
                "/projects/{}/requirements/{}",
                project_id, requirement_id
            )))
            .send()
            .await?;

        Ok(())
    }

    fn append_to_url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }
}
