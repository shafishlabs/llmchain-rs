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
use llmchain_loaders::DocumentSplitter;
use llmchain_loaders::TextLoader;
use llmchain_loaders::TextSplitter;
use opendal::services::Fs;
use opendal::BlockingOperator;
use opendal::Operator;

#[test]
fn test_text_splitter_default() -> Result<()> {
    // testdata dir.
    let curdir = std::env::current_dir()?.to_str().unwrap().to_string();
    let testdata_dir = format!("{}/tests/testdata", curdir);

    // Operator.
    let mut builder = Fs::default();
    builder.root(&testdata_dir);
    let op: BlockingOperator = Operator::new(builder)?.finish().blocking();

    // Load
    let text_loader = TextLoader::create(op);
    let documents = text_loader.load("text/example.txt")?;

    let text_splitter = TextSplitter::create();
    let documents = text_splitter.split_documents(&documents)?;

    // Check.
    let mut mint = Mint::new(&testdata_dir);
    let golden_path = "text/example_txt_splitter_default.golden";
    let mut file = mint.new_goldenfile(golden_path)?;
    for (i, doc) in documents.iter().enumerate() {
        writeln!(
            file,
            "part={}, len={}, chunk_size={}, path={}",
            i,
            doc.content.len(),
            text_splitter.splitter_chunk_size,
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

#[test]
fn test_text_splitter_10() -> Result<()> {
    // testdata dir.
    let curdir = std::env::current_dir()?.to_str().unwrap().to_string();
    let testdata_dir = format!("{}/tests/testdata", curdir);

    // Operator.
    let mut builder = Fs::default();
    builder.root(&testdata_dir);
    let op: BlockingOperator = Operator::new(builder)?.finish().blocking();

    // Load
    let text_loader = TextLoader::create(op);
    let documents = text_loader.load("text/example.txt")?;

    let text_splitter = TextSplitter::create().with_chunk_size(10);
    let documents = text_splitter.split_documents(&documents)?;

    // Check.
    assert_eq!(documents.len(), 2);

    // Check.
    let mut mint = Mint::new(&testdata_dir);
    let golden_path = "text/example_txt_splitter_chunk_10.golden";
    let mut file = mint.new_goldenfile(golden_path)?;
    for (i, doc) in documents.iter().enumerate() {
        writeln!(
            file,
            "part={}, len={}, chunk_size={}, path={}",
            i,
            doc.content.len(),
            text_splitter.splitter_chunk_size,
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
