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
use llmchain::DocumentLoader;
use llmchain::DocumentPath;
use llmchain::DocumentSplitter;
use llmchain::GithubPRDiffSplitter;
use llmchain::GithubPRLoader;

#[tokio::test]
async fn test_github_pr_splitter_default() -> Result<()> {
    let token = std::env::var("L_GITHUB_TOKEN").expect("L_GITHUB_TOKEN is not set");
    // testdata dir.
    let curdir = std::env::current_dir()?.to_str().unwrap().to_string();
    let testdata_dir = format!("{}/tests/testdata", curdir);

    // Load
    let github_pr_loader = GithubPRLoader::create("datafuselabs", "databend", &token);
    let documents = github_pr_loader
        .load(DocumentPath::from_list(vec![
            11450, 11451, 11452, 11453, 11454, 11455, 11456, 11457, 11458, 11459,
        ]))
        .await?;

    let github_pr_splitter =
        GithubPRDiffSplitter::create().with_skips(vec!["**/*.txt".to_string()]);
    let documents = github_pr_splitter.split_documents(&documents)?;

    // Check.
    let mut mint = Mint::new(&testdata_dir);
    let golden_path = "github/github_pr_splitter_default.golden";
    let mut file = mint.new_goldenfile(golden_path)?;
    for (i, doc) in documents.iter().enumerate() {
        writeln!(
            file,
            "part={}, len={}, chunk_size={}, md5={}, path:{}",
            i,
            doc.content.len(),
            github_pr_splitter.splitter_chunk_size,
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
