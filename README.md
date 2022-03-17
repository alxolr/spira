# spira@0.0.1

Inflectra Spira Rust Client

## Usage

```rust
use spira::{SpiraClient, resources::project::ProjectDto};

let api_key = env::var("SPIRA_API_KEY")?;
let username = env::var("SPIRA_USERNAME")?;
let base_url = env::var("SPIRA_API_URL")?;

let spira_client = SpiraClient::new(&base_url, &api_key, &username)?;
let projects: Vec<ProjectDto> = spira_client.project.list().await?;
```
