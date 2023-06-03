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
use databend_driver::new_connection;
use llmchain_common::escape_sql_string;
use log::info;
use tokio_stream::StreamExt;

use crate::llm::EmbeddingResult;
use crate::llm::GenerateResult;
use crate::llm::LLM;

pub struct DatabendLLM {
    dsn: String,
}

impl DatabendLLM {
    pub fn create(dsn: &str) -> Self {
        DatabendLLM {
            dsn: dsn.to_string(),
        }
    }
}

#[async_trait::async_trait]
impl LLM for DatabendLLM {
    async fn embedding(&self, inputs: Vec<String>) -> Result<EmbeddingResult> {
        let conn = new_connection(&self.dsn)?;
        let mut embeddings = vec![];
        for (i, input) in inputs.iter().enumerate() {
            let now = std::time::Instant::now();
            type RowResult = (String,);
            let mut rows = conn
                .query_iter(&format!(
                    "SELECT ai_embedding_vector('{}')",
                    escape_sql_string(input)
                ))
                .await?;
            while let Some(row) = rows.next().await {
                let row: RowResult = row?.try_into()?;
                let array_vec: Vec<f32> = serde_json::from_str(&row.0)?;
                info!(
                    "embedding {}/{},  time: {:?}",
                    i + 1,
                    inputs.len(),
                    now.elapsed()
                );
                embeddings.push(array_vec);
            }
        }

        Ok(EmbeddingResult {
            prompt_tokens: 0,
            total_tokens: 0,
            embeddings,
        })
    }

    async fn generate(&self, input: &str) -> Result<GenerateResult> {
        let conn = new_connection(&self.dsn)?;
        let row = conn
            .query_row(&format!(
                "SELECT ai_text_completion('{}')",
                escape_sql_string(input)
            ))
            .await?;

        let mut generation = "".to_string();

        if let Some(res) = row {
            type RowResult = (String,);
            let row: RowResult = res.try_into()?;
            generation = row.0;
        }

        Ok(GenerateResult {
            prompt_tokens: 0,
            completion_tokens: 0,
            total_tokens: 0,
            generation,
        })
    }
}
