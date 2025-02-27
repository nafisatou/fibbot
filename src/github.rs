use reqwest::Client;
use serde_json::json;
use std::error::Error;

pub async fn post_comment_to_pr(pr_url: &str, token: &str, comment: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    
    let res = client
        .post(format!("{}/comments", pr_url))
        .header("Authorization", format!("token {}", token))
        .json(&json!({ "body": comment }))
        .send()
        .await?;
    
    if res.status().is_success() {
        println!("Successfully posted comment to PR");
        Ok(())
    } else {
        Err(format!("Failed to post comment: {}", res.status()).into())
    }
}
