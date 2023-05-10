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
use async_openai::types::ChatCompletionRequestMessageArgs;
use async_openai::types::CreateChatCompletionRequestArgs;
use async_openai::types::CreateEmbeddingRequestArgs;
use async_openai::types::Role;
use async_openai::Client;

use crate::llm::LLM;
use crate::openai::OpenAIConfig;

pub struct OpenAI {
    conf: OpenAIConfig,
    client: Client,
}

impl OpenAI {
    pub fn create(conf: OpenAIConfig) -> Self {
        let client = Client::new()
            .with_api_base(&conf.openai_api_base)
            .with_api_key(&conf.openai_api_key);

        OpenAI { conf, client }
    }
}

#[async_trait::async_trait]
impl LLM for OpenAI {
    async fn embedding(&self, inputs: Vec<String>) -> Result<Vec<Vec<f32>>> {
        let request = CreateEmbeddingRequestArgs::default()
            .model(&self.conf.embedding_model.to_string())
            .input(inputs)
            .build()?;

        let response = self.client.embeddings().create(request).await?;
        let mut result = Vec::with_capacity(response.data.len());
        for embedding in &response.data {
            result.push(embedding.embedding.clone());
        }

        Ok(result)
    }

    async fn generate<S: Into<String> + Send>(&self, input: S) -> Result<String> {
        let request = CreateChatCompletionRequestArgs::default()
            .max_tokens(self.conf.max_token as u16)
            .model(&self.conf.generate_model.to_string())
            .messages([ChatCompletionRequestMessageArgs::default()
                .role(Role::Assistant)
                .content(input)
                .build()?])
            .build()?;

        let response = self.client.chat().create(request).await?;
        if !response.choices.is_empty() {
            return Ok(response.choices[0].message.content.clone());
        }

        Ok("".to_string())
    }
}
