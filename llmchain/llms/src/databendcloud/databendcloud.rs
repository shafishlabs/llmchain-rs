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

use crate::llm::EmbeddingResult;
use crate::llm::GenerateResult;
use crate::llm::LLM;

pub struct DatabendCloud {
    dsn: String,
}

impl DatabendCloud {
    pub fn create(dsn: &str) -> Self {
        DatabendCloud {
            dsn: dsn.to_string(),
        }
    }
}

#[async_trait::async_trait]
impl LLM for DatabendCloud {
    async fn embedding(&self, inputs: Vec<String>) -> Result<EmbeddingResult> {
        todo!()
    }

    async fn generate(&self, input: &str) -> Result<GenerateResult> {
        todo!()
    }
}
