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

use crate::DatabendLLM;
use crate::Documents;
use crate::Embedding;
use crate::LLM;

pub struct DatabendEmbedding {
    llm: Arc<DatabendLLM>,
}

impl DatabendEmbedding {
    pub fn create(dsn: &str) -> Self {
        DatabendEmbedding {
            llm: DatabendLLM::create(dsn),
        }
    }
}

#[async_trait::async_trait]
impl Embedding for DatabendEmbedding {
    async fn embed_query(&self, input: &str) -> Result<Vec<f32>> {
        let inputs = vec![input.to_string()];
        let result = self.llm.embedding(inputs).await?;

        if result.embeddings.is_empty() {
            Ok(vec![])
        } else {
            Ok(result.embeddings[0].clone())
        }
    }

    async fn embed_documents(&self, inputs: &Documents) -> Result<Vec<Vec<f32>>> {
        let inputs = inputs.iter().map(|x| x.content).collect::<Vec<_>>();
        let result = self.llm.embedding(inputs).await?;

        Ok(result.embeddings)
    }
}
