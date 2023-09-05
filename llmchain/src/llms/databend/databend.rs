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

use anyhow::anyhow;
use anyhow::Result;
use databend_driver::Client;
use log::info;
use tokio_stream::StreamExt;

use crate::escape_sql_string;
use crate::EmbeddingResult;
use crate::GenerateResult;
use crate::LLM;

pub struct DatabendLLM {
    client: Client,
}

impl DatabendLLM {
    pub fn create(dsn: &str) -> Arc<Self> {
        Arc::new(DatabendLLM {
            client: Client::new(dsn.to_string()),
        })
    }
}

#[async_trait::async_trait]
impl LLM for DatabendLLM {
    async fn embedding(&self, inputs: Vec<String>) -> Result<EmbeddingResult> {
        let conn = self.client.get_conn().await?;
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
                let row: RowResult = row?.try_into().map_err(|e: String| anyhow!(e))?;
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
        let conn = self.client.get_conn().await?;
        let row = conn
            .query_row(&format!(
                "SELECT ai_text_completion('{}')",
                escape_sql_string(input)
            ))
            .await?;

        let generation = match row {
            Some(row) => {
                let (gen,): (String,) = row.try_into().map_err(|e: String| anyhow!(e))?;
                gen
            }
            None => "".to_string(),
        };

        Ok(GenerateResult {
            prompt_tokens: 0,
            completion_tokens: 0,
            total_tokens: 0,
            generation,
        })
    }
}
