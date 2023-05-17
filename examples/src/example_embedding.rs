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
use llmchain_embeddings::Embedding;
use llmchain_embeddings::OpenAIEmbedding;
use llmchain_loaders::Document;

/// EXPORT OPENAI_API_KEY=<your-openai-api-key>
/// cargo run --bin example_embedding
#[tokio::main]
async fn main() -> Result<()> {
    let api_key = std::env::var("OPENAI_API_KEY")
        .map_err(|_| {
            "OPENAI_API_KEY is empty, please EXPORT OPENAI_API_KEY=<your-openai-api-key>"
                .to_string()
        })
        .unwrap();

    // Sample documents.
    let documents = vec![
        Document::create("", "hello"),
        Document::create("", "llmchain.rs"),
    ];

    // create openai embedding.
    let embeddings = OpenAIEmbedding::create(&api_key);

    // embedding documents.
    let document_result = embeddings.embed_documents(documents).await?;
    println!("{:?}", document_result);

    Ok(())
}
