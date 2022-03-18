use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::Response;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct TaskDto {
    /// The id of the task
    #[serde(rename = "TaskId")]
    pub task_id: Option<u64>,

    /// The id of the status of the task
    #[serde(rename = "TaskStatusId")]
    pub task_status_id: Option<u64>,

    /// The id of the type of the task
    #[serde(rename = "TaskTypeId")]
    pub task_type_id: Option<u64>,

    /// The of the folder the task is stored in (null for root)
    #[serde(rename = "TaskFolderId")]
    pub task_folder_id: Option<u64>,

    /// The id of the parent requirement that the task belongs to
    #[serde(rename = "RequirementId")]
    pub requirement_id: Option<u64>,

    /// The id of the release/iteration that the task is scheduled for
    #[serde(rename = "ReleaseId")]
    pub release_id: Option<u64>,

    /// The id of the user that originally created the task If no value is provided, the authenticated user is used instead
    #[serde(rename = "CreatorId")]
    pub creator_id: Option<u64>,

    /// The id of the user that the task is assigned-to
    #[serde(rename = "OwnerId")]
    pub owner_id: Option<u64>,

    /// The id of the priority of the task
    #[serde(rename = "TaskPriorityId")]
    pub task_priority_id: Option<u64>,

    /// The name of the task
    #[serde(rename = "Name")]
    pub name: Option<String>,

    /// The detailed description of the task
    #[serde(rename = "Description")]
    pub description: Option<String>,

    /// The date/time that the task was originally created
    /// { "CreationDate": "2022-03-14T11:28:07.240Z" },
    #[serde(rename = "CreationDate")]
    pub creation_date: Option<String>,

    /// The date/time that the task was last modified This field needs to match the values retrieved to ensure data-concurrency
    #[serde(rename = "LastUpdateDate")]
    pub last_update_date: Option<String>,

    /// The scheduled start date for the task
    #[serde(rename = "StartDate")]
    pub start_date: Option<String>,

    /// The scheduled end date for the task
    #[serde(rename = "EndDate")]
    pub end_date: Option<String>,

    /// The originally estimated effort (in minutes) of the task
    #[serde(rename = "EstimatedEffort")]
    pub estimated_effort: Option<u64>,

    /// The actual effort expended so far (in minutes) for the task
    #[serde(rename = "ActualEffort")]
    pub actual_effort: Option<u64>,

    /// The effort remaining as reported by the developer
    #[serde(rename = "RemainingEffort")]
    pub remaining_effort: Option<u64>,

    /// The id of the project that the artifact belongs to The current project is always used for Insert operations for security reasons
    #[serde(rename = "ProjectId")]
    pub project_id: u64,

    /// The datetime used to track optimistic concurrency to prevent edit conflicts
    #[serde(rename = "ConcurrencyDate")]
    pub concurrency_date: Option<String>,
}

pub struct TaskClient<'a> {
    client: Client,
    base_url: &'a str,
}

impl<'a> TaskClient<'a> {
    pub fn new(client: Client, base_url: &'a str) -> Self {
        TaskClient { client, base_url }
    }

    /// Retrieves all tasks owned by the currently authenticated user
    pub async fn list_my(&self) -> Response<Vec<TaskDto>> {
        let tasks = self
            .client
            .get(self.append_to_url("/tasks"))
            .send()
            .await?
            .json::<Vec<TaskDto>>()
            .await?;

        Ok(tasks)
    }

    /// Retrieves a single task in the system
    pub async fn get(&self, project_id: u64, task_id: u64) -> Response<TaskDto> {
        let path = &format!("/projects/{}/tasks/{}", project_id, task_id);

        let task = self
            .client
            .get(self.append_to_url(path))
            .send()
            .await?
            .json::<TaskDto>()
            .await?;

        Ok(task)
    }

    /// Creates a new task in the system
    pub async fn create(&self, project_id: u64, task: TaskDto) -> Response<TaskDto> {
        let json_task = serde_json::to_string(&task)?;
        let task = self
            .client
            .post(self.append_to_url(&format!("/projects/{}/tasks", project_id)))
            .body(json_task)
            .send()
            .await?
            .json::<TaskDto>()
            .await?;

        Ok(task)
    }

    /// Updates a task in the system
    pub async fn update(&self, project_id: u64, task: TaskDto) -> Response<()> {
        let json_task = serde_json::to_string(&task)?;
        self.client
            .put(self.append_to_url(&format!("/projects/{}/tasks", project_id)))
            .body(json_task)
            .send()
            .await?;

        Ok(())
    }

    /// Updates a task in the system
    pub async fn delete(&self, project_id: u64, task_id: u64) -> Response<()> {
        self.client
            .delete(self.append_to_url(&format!("/projects/{}/tasks/{}", project_id, task_id)))
            .send()
            .await?;

        Ok(())
    }

    fn append_to_url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }
}
