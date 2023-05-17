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
use llmchain_embeddings::Embedding;
use llmchain_loaders::Document;

#[async_trait::async_trait]
pub trait VectorStore: Send + Sync {
    async fn init(&self, embeddings: Arc<dyn Embedding>) -> Result<()>;
    async fn add_texts(&self, inputs: Vec<String>) -> Result<()>;
    async fn add_documents(&self, inputs: Vec<Document>) -> Result<()>;
    async fn similarity_search(&self, query: &str) -> Result<Vec<Document>>;
}
