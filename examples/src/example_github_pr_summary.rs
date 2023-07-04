// Copyright 2023 Shafish Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use anyhow::Result;
use env_logger::Env;
use llmchain::DatabendLLM;
use llmchain::DocumentLoader;
use llmchain::DocumentPath;
use llmchain::DocumentSplitter;
use llmchain::GithubPRDiffSplitter;
use llmchain::GithubPRLoader;
use llmchain::GithubPRSummary;
use llmchain::Summarize;
use llmchain_examples::kit::handle_repl;
use llmchain_examples::kit::ReplAsyncCallback;
use log::info;
use url::Url;

/// EXPORT DATABEND_DSN=<your-databend-dsn>
/// cargo run --bin example_github_pr_summary
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let callback: Box<ReplAsyncCallback> = Box::new(|input| Box::pin(github_pr_summary(input)));
    handle_repl("pr> ", callback).await?;

    Ok(())
}

async fn github_pr_summary(pr: String) -> Result<String> {
    if pr.is_empty() {
        return Ok("Input Github PR URL which you want to summary".to_string());
    }

    let (owner, repo, pull_id) = parse_github_pr(&pr)?;
    let databend_dsn = std::env::var("DATABEND_DSN")
        .map_err(|_| {
            "DATABEND_DSN is empty, please EXPORT DATABEND_DSN=<your-databend-dsn>".to_string()
        })
        .unwrap();

    let github_token = std::env::var("GITHUB_TOKEN").unwrap_or("".to_string());

    let documents = GithubPRLoader::create(&owner, &repo, &github_token)
        .load(DocumentPath::from_list(vec![pull_id]))
        .await?;

    let documents = GithubPRDiffSplitter::create()
        .with_chunk_size(8000)
        .split_documents(&documents)
        .unwrap();

    let databend_llm = DatabendLLM::create(&databend_dsn);
    let summary = GithubPRSummary::create(databend_llm);
    summary.add_documents(&documents).await?;
    let pr_summary = summary.final_summary().await?;

    let final_summary = format!(
        "{}\nTokens:{}\n## Summary(By llmchain.rs)\n{}",
        pr,
        summary.tokens(),
        pr_summary
    );
    Ok(final_summary)
}

fn parse_github_pr(url: &str) -> Result<(String, String, usize)> {
    let parsed_url = Url::parse(url)?;

    let mut segments = parsed_url.path_segments().expect("path segments");

    let owner = segments.next().expect("owner").to_string();
    info!("owner: {}", owner);
    let repo = segments.next().expect("repo").to_string();
    info!("repo: {}", repo);

    // Ignoring the next segment because it's "pull" or "pulls"
    let _ = segments.next();

    let pull_id_str = segments.next().expect("pr id").to_string();
    let pull_id: usize = pull_id_str.parse().expect("parse pr id error");

    Ok((owner, repo, pull_id))
}
