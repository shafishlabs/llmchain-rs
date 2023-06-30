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

use std::sync::Arc;

use anyhow::Result;
use env_logger::Env;
use llmchain::DatabendEmbedding;
use llmchain::DatabendVectorStore;
use llmchain::Document;
use llmchain::Documents;
use llmchain::VectorStore;
use log::info;

/// EXPORT DATABEND_DSN=<your-databend-dsn>
/// cargo run --bin example_vector_store
#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let dsn = std::env::var("DATABEND_DSN")
        .map_err(|_| {
            "DATABEND_DSN is empty, please EXPORT DATABEND_DSN=<your-databend-dsn>".to_string()
        })
        .unwrap();

    // Sample documents.
    let documents = Documents::from(vec![
        Document::create("1.md", "hello"),
        Document::create("2.md", "llmchain.rs"),
    ]);

    // create embedding.
    let databend_embedding = Arc::new(DatabendEmbedding::create(&dsn));

    // create databend vector store.
    let databend = DatabendVectorStore::create(&dsn, databend_embedding);
    databend.init().await?;

    // add documents to vector store.
    let uuids = databend.add_documents(&documents).await?;
    info!("embedding uuids:{:?}", uuids);

    // query a similarity document.
    let query = "llmchain";
    let similarities = databend.similarity_search("llmchain", 1).await?;
    info!("query:{}, similarity documents:{:?}", query, similarities);

    Ok(())
}
