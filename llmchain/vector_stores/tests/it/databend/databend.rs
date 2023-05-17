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
use llmchain_embeddings::OpenAIEmbedding;
use llmchain_loaders::Document;
use llmchain_vector_stores::DatabendVectorStore;
use llmchain_vector_stores::VectorStore;

#[tokio::test]
async fn test_vector_stores_databend() -> Result<()> {
    let api_key = std::env::var("OPENAI_API_KEY").unwrap();
    let dsn = std::env::var("DATABEND_DSN").unwrap();

    let openai_embedding = Arc::new(OpenAIEmbedding::create(&api_key));
    let databend = DatabendVectorStore::create(&dsn, openai_embedding);
    databend.init().await?;

    let documents = vec![
        Document::create("1.md", "hello"),
        Document::create("2.md", "llmchai.rs"),
    ];
    let result = databend.add_documents(documents).await?;
    assert_eq!(result.len(), 2);

    let similarities = databend.similarity_search("llmchain", 1).await?;
    assert_eq!(similarities.len(), 1);

    Ok(())
}
