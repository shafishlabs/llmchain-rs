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
use llmchain_embeddings::DatabendEmbedding;
use llmchain_embeddings::Embedding;
use llmchain_loaders::Document;

#[tokio::test]
async fn test_embedding_databend() -> Result<()> {
    let dsn = std::env::var("DATABEND_DSN").expect("DATABEND_DSN is not set");

    // embeddings query.
    {
        let embeddings = DatabendEmbedding::create(&dsn);
        let query_result = embeddings.embed_query("hello").await?;
        assert_eq!(query_result.len(), 1536);
    }

    // embeddings documents.
    {
        let embeddings = DatabendEmbedding::create(&dsn);
        let documents = vec![
            Document::create("", "hello"),
            Document::create("", "llmchain.rs"),
        ];
        let document_result = embeddings.embed_documents(documents).await?;
        assert_eq!(document_result.len(), 2);
        assert_eq!(document_result[0].len(), 1536);
    }

    Ok(())
}
