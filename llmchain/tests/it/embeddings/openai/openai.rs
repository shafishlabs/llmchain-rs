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
use llmchain::Document;
use llmchain::Documents;
use llmchain::Embedding;
use llmchain::OpenAI;
use llmchain::OpenAIEmbedding;

#[ignore]
#[tokio::test]
async fn test_embedding_openai() -> Result<()> {
    let api_key = std::env::var("OPENAI_API_KEY").unwrap_or("".to_string());

    // embeddings query.
    {
        let embeddings = OpenAIEmbedding::create(OpenAI::create(&api_key));
        let query_result = embeddings.embed_query("hello").await?;
        assert_eq!(query_result.len(), 1536);
    }

    // embeddings documents.
    {
        let embeddings = OpenAIEmbedding::create(OpenAI::create(&api_key));
        let documents = Documents::from(vec![
            Document::create("", "hello"),
            Document::create("", "llmchain.rs"),
        ]);
        let document_result = embeddings.embed_documents(&documents).await?;
        assert_eq!(document_result.len(), 2);
        assert_eq!(document_result[0].len(), 1536);
    }

    Ok(())
}
