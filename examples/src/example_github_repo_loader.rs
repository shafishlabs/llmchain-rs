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
use llmchain_loaders::DocumentLoader;
use llmchain_loaders::DocumentPath;
use llmchain_loaders::GithubRepoLoader;
use log::info;

/// cargo run --bin example_github_repo_loader
#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // documents
    let documents = GithubRepoLoader::create()
        .load(DocumentPath::from_string(
            "https://github.com/shafishlabs/llmchain.rs",
        ))
        .await?;

    info!("{:?}", documents);

    Ok(())
}