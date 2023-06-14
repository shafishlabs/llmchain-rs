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
use llmchain_test_kits::handle_repl;
use llmchain_test_kits::AsyncCallback;

/// EXPORT GITHUB_TOKEN=<your-personal-github-token>
/// EXPORT DATABEND_DSN=<your-databend-dsn>
/// cargo run --bin example_github_pr_summary
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let callback: Box<AsyncCallback> = Box::new(|input| Box::pin(fake_pr_summary(input)));
    handle_repl("pr: ", callback).await?;

    Ok(())
}

async fn fake_pr_summary(pr: String) -> String {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let _databend_dsn = std::env::var("DATABEND_DSN")
        .map_err(|_| {
            "DATABEND_DSN is empty, please EXPORT DATABEND_DSN=<your-databend-dsn>".to_string()
        })
        .unwrap();

    let _github_token = std::env::var("GITHUB_TOKEN")
        .map_err(|_| {
            "GITHUB_TOKEN is empty, please EXPORT GITHUB_TOKEN=<your-personal-github-token>"
                .to_string()
        })
        .unwrap();

    // let documents = GithubPRLoader::create("datafuselabs", "databend", &github_token)
    // .load(DocumentPath::from_list(vec![11634]))
    // .await?;
    //
    // let documents = GithubPRDiffSplitter::create()
    // .with_chunk_size(8000)
    // .split_documents(&documents)?;
    //

    format!("pr: {}", pr)
}
