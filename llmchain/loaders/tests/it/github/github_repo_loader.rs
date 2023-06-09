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
use llmchain_loaders::DocumentLoader;
use llmchain_loaders::DocumentPath;
use llmchain_loaders::GithubRepoLoader;

#[tokio::test]
async fn test_github_repo_loader() -> Result<()> {
    env_logger::init();

    // Load
    let github_repo_loader = GithubRepoLoader::create("shafishlabs", "llmchain.rs", "");
    let documents = github_repo_loader
        .load(DocumentPath::Str("".to_string()))
        .await?;

    assert!(documents.len() > 10);
    assert!(documents[0].path.starts_with("https://github.com"));

    Ok(())
}
