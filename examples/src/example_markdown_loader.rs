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
use llmchain::DirectoryLoader;
use llmchain::DocumentLoader;
use llmchain::DocumentPath;
use llmchain::LocalDisk;
use llmchain::MarkdownLoader;
use log::info;

/// cargo run --bin example_markdown_loader
#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // dir.
    let curdir = std::env::current_dir()?.to_str().unwrap().to_string();
    let testdata_dir = format!("{}/examples/testdata", curdir);
    let directory_dir = format!("{}/markdowns/", testdata_dir);
    info!("{}", directory_dir);

    // Loader from local disk.
    let local_disk = LocalDisk::create()?;

    // Markdown loader with a local disk.
    let markdown_loader = MarkdownLoader::create(local_disk);

    // Directory loader.
    let directory_loader =
        DirectoryLoader::create(LocalDisk::create()?).with_loader("**/*.md", markdown_loader);

    // loader all documents.
    let documents = directory_loader
        .load(DocumentPath::from_string(&directory_dir))
        .await?;
    info!("{:?}", documents);

    Ok(())
}
