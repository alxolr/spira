# spira@0.0.1

Inflectra Spira Rust Client

## Usage

```rust
use spira::{resources::project::ProjectDto, SpiraClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("SPIRA_API_KEY")?;
    let username = env::var("SPIRA_USERNAME")?;
    let base_url = env::var("SPIRA_API_URL")?;

    let spira_client = SpiraClient::new(&base_url, &api_key, &username)?;
    let projects = spira_client.project.list().await?;

    println!("{:#?}", projects);
    Ok(())
}
```
