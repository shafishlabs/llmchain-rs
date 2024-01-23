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
use llmchain::LocalDisk;
use llmchain::MarkdownLoader;
use llmchain::MarkdownSplitter;

#[tokio::test]
async fn test_markdown_splitter_default() -> Result<()> {
    // testdata dir.
    let curdir = std::env::current_dir()?.to_str().unwrap().to_string();
    let testdata_dir = format!("{}/tests/testdata/loaders", curdir);
    let markdown_file = format!("{}/markdown/copy.md", testdata_dir);

    // Load
    let markdown_loader = MarkdownLoader::create(LocalDisk::create()?);
    let documents = markdown_loader
        .load(DocumentPath::from_string(&markdown_file))
        .await?;

    let markdown_splitter = MarkdownSplitter::create();
    let documents = markdown_splitter.split_documents(&documents)?;

    // Check.
    let mut mint = Mint::new(&testdata_dir);
    let golden_path = "markdown/copy_md_splitter_default.golden";
    let mut file = mint.new_goldenfile(golden_path)?;
    for (i, doc) in documents.iter().enumerate() {
        writeln!(
            file,
            "part={}, len={}, chunk_size={}, md5={}",
            i,
            doc.content.len(),
            markdown_splitter.splitter_chunk_size,
            doc.content_md5
        )?;
        writeln!(
            file,
            "------------------------------------------------------------"
        )?;
        writeln!(file, "{}", doc.content)?;
    }

    Ok(())
}

#[tokio::test]
async fn test_markdown_splitter_100() -> Result<()> {
    // testdata dir.
    let curdir = std::env::current_dir()?.to_str().unwrap().to_string();
    let testdata_dir = format!("{}/tests/testdata/loaders", curdir);
    let markdown_file = format!("{}/markdown/copy.md", testdata_dir);

    // Load
    let markdown_loader = MarkdownLoader::create(LocalDisk::create()?);
    let documents = markdown_loader
        .load(DocumentPath::from_string(&markdown_file))
        .await?;

    let markdown_splitter = MarkdownSplitter::create().with_chunk_size(100);
    let documents = markdown_splitter.split_documents(&documents)?;

    // Check.
    assert_eq!(documents.len(), 14);

    // Check.
    let mut mint = Mint::new(&testdata_dir);
    let golden_path = "markdown/copy_md_splitter_chunk_100.golden";
    let mut file = mint.new_goldenfile(golden_path)?;
    for (i, doc) in documents.iter().enumerate() {
        writeln!(
            file,
            "part={}, len={}, chunk_size={}, md5={}",
            i,
            doc.content.len(),
            markdown_splitter.splitter_chunk_size,
            doc.content_md5
        )?;
        writeln!(
            file,
            "------------------------------------------------------------"
        )?;
        writeln!(file, "{}", doc.content)?;
    }

    Ok(())
}


#[tokio::test]
async fn test_markdown_splitter_custom_separator() -> Result<()> {
    // testdata dir.
    let curdir = std::env::current_dir()?.to_str().unwrap().to_string();
    let testdata_dir = format!("{}/tests/testdata/loaders", curdir);
    let markdown_file = format!("{}/markdown/copy-hyphen.md", testdata_dir);

    // Load
    let markdown_loader = MarkdownLoader::create(LocalDisk::create()?);
    let documents = markdown_loader
        .load(DocumentPath::from_string(&markdown_file))
        .await?;

    let markdown_splitter = MarkdownSplitter::create().with_separators(vec![
        "\n- ## ".to_string(),
        "\n- ### ".to_string(),
        "\n- #### ".to_string(),
        "\n- ##### ".to_string(),
        "\n- ###### ".to_string()
    ]);
    let documents = markdown_splitter.split_documents(&documents)?;

    // Check.
    let mut mint = Mint::new(&testdata_dir);
    let golden_path = "markdown/copy_md_splitter_custom_separator.golden";
    let mut file = mint.new_goldenfile(golden_path)?;
    for (i, doc) in documents.iter().enumerate() {
        writeln!(
            file,
            "part={}, len={}, chunk_size={}, md5={}",
            i,
            doc.content.len(),
            markdown_splitter.splitter_chunk_size,
            doc.content_md5
        )?;
        writeln!(
            file,
            "------------------------------------------------------------"
        )?;
        writeln!(file, "{}", doc.content)?;
    }

    Ok(())
}

