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
use llmchain_loaders::document::DocumentLoader;
use llmchain_loaders::document::DocumentSettings;
use llmchain_loaders::markdown::Markdown;
use llmchain_loaders::markdown::MarkdownSplitter;
use llmchain_loaders::splitter::TextSplitter;
use opendal::services::Fs;
use opendal::Operator;

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_markdown() -> Result<()> {
    // testdata dir.
    let curdir = std::env::current_dir()?.to_str().unwrap().to_string();
    let testdata_dir = format!("{}/tests/testdata", curdir);

    // Operator.
    let mut builder = Fs::default();
    builder.root(&testdata_dir);
    let op: Operator = Operator::new(builder)?.finish();

    // Load
    let markdown = Markdown::create(op.clone());
    let documents = markdown.load("markdowns/copy.md").await?;

    let settings = DocumentSettings {
        splitter_chunk_size: 400,
    };
    let markdown_splitter = MarkdownSplitter::create(&settings);
    let documents = markdown_splitter.split_documents(&documents)?;

    // Check.
    let mut mint = Mint::new(&testdata_dir);
    let golden_path = "markdowns/copy_md_splitter.txt";
    let mut file = mint.new_goldenfile(golden_path)?;
    for (i, doc) in documents.iter().enumerate() {
        writeln!(
            file,
            "part={}, len={}, chunk_size={}, path={}",
            i,
            doc.content.len(),
            settings.splitter_chunk_size,
            doc.meta.path
        )?;
        writeln!(
            file,
            "------------------------------------------------------------"
        )?;
        writeln!(file, "{}", doc.content)?;
    }

    Ok(())
}
