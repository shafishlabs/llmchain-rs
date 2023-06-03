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

use std::io::Write;

use anyhow::Result;
use goldenfile::Mint;
use llmchain_loaders::DocumentLoader;
use llmchain_loaders::DocumentPath;
use llmchain_loaders::GithubPRLoader;

#[tokio::test]
async fn test_github_pr_loader() -> Result<()> {
    let token = std::env::var("GITHUB_TOKEN").unwrap();
    // testdata dir.
    let curdir = std::env::current_dir()?.to_str().unwrap().to_string();
    let testdata_dir = format!("{}/tests/testdata", curdir);

    // Load
    let github_pr_loader = GithubPRLoader::create("datafuselabs", "databend", &token);
    let documents = github_pr_loader
        .load(DocumentPath::from_range(11450, 11460))
        .await?;

    // Check.
    let mut mint = Mint::new(&testdata_dir);
    let golden_path = "github/github_pr_loader.golden";
    let mut file = mint.new_goldenfile(golden_path)?;
    for (i, doc) in documents.iter().enumerate() {
        writeln!(
            file,
            "part={}, len={}, md5={}, path:{}",
            i,
            doc.content.len(),
            doc.content_md5,
            doc.path
        )?;
        writeln!(
            file,
            "------------------------------------------------------------"
        )?;
        writeln!(file, "{}", doc.content)?;
    }

    Ok(())
}
