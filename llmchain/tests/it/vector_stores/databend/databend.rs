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
use llmchain::DatabendEmbedding;
use llmchain::DatabendVectorStore;
use llmchain::Document;
use llmchain::Documents;
use llmchain::VectorStore;

#[tokio::test]
async fn test_vector_stores_databend() -> Result<()> {
    let dsn = std::env::var("DATABEND_DSN").expect("DATABEND_DSN is not set");

    let databend_embedding = Arc::new(DatabendEmbedding::create(&dsn));
    let databend = DatabendVectorStore::create(&dsn, databend_embedding);
    databend.init().await?;

    let documents = Documents::from(vec![
        Document::create("1.md", "hello"),
        Document::create("2.md", "llmchain.rs"),
    ]);
    let result = databend.add_documents(&documents).await?;
    assert_eq!(result.len(), 2);

    let similarities = databend.similarity_search("llmchain", 1).await?;
    assert_eq!(similarities.len(), 1);

    let expect_document = Document {
        path: "2.md".to_string(),
        content: "llmchain.rs".to_string(),
        content_md5: "033d6bd60a5237d54fa8331dd2ca1325".to_string(),
    };

    let actual_document = similarities[0].clone();

    assert_eq!(expect_document, actual_document);

    Ok(())
}
