//! SpiraClient
//! Inflectra Spira Rest client implementation in rust

pub mod resources;

use reqwest::{
    self,
    header::{HeaderMap, HeaderValue},
    Client,
};
use resources::requirement::RequirementClient;
use std::time::Duration;

use self::resources::{project::ProjectClient, task::TaskClient, user::UserClient};

/// The main client, contains child clients for each resource type like:
/// TaskClient, ProjectClient, UserClient
pub struct SpiraClient<'a> {
    pub task: TaskClient<'a>,
    pub project: ProjectClient<'a>,
    pub user: UserClient<'a>,
    pub requirement: RequirementClient<'a>,
}

type Response<T> = Result<T, Box<dyn std::error::Error>>;

/// ## Usage Example
/// ```rust
/// use spira::{resources::project::ProjectDto, SpiraClient};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let api_key = env::var("SPIRA_API_KEY")?;
///     let username = env::var("SPIRA_USERNAME")?;
///     let base_url = env::var("SPIRA_API_URL")?;
///
///     let spira_client = SpiraClient::new(&base_url, &api_key, &username)?;
///     let projects = spira_client.project.list().await?;
///
///     println!("{:#?}", projects);
///
///     Ok(())
/// }
/// ```
impl<'a> SpiraClient<'a> {
    pub fn new(base_url: &'a str, api_key: &str, username: &str) -> Response<Self> {
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_str("application/json")?);
        headers.insert("Content-type", HeaderValue::from_str("application/json")?);
        headers.insert("api-key", HeaderValue::from_str(api_key)?);
        headers.insert("username", HeaderValue::from_str(username)?);

        let client = Client::builder()
            .connect_timeout(Duration::from_secs(10))
            .default_headers(headers)
            .build()?;

        let task = TaskClient::new(client.clone(), base_url);
        let project = ProjectClient::new(client.clone(), base_url);
        let requirement = RequirementClient::new(client.clone(), base_url);
        let user = UserClient::new(client, base_url);

        Ok(SpiraClient {
            task,
            requirement,
            project,
            user,
        })
    }
}
