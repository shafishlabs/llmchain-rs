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
use async_openai::config::OpenAIConfig;
use async_openai::types::ChatCompletionRequestMessageArgs;
use async_openai::types::CreateChatCompletionRequestArgs;
use async_openai::types::CreateEmbeddingRequestArgs;
use async_openai::types::Role;
use async_openai::Client;
use parking_lot::RwLock;

use crate::llm::EmbeddingResult;
use crate::llm::GenerateResult;
use crate::llm::LLM;

pub enum OpenAIEmbeddingModel {
    TextEmbeddingAda002,
}

impl ToString for OpenAIEmbeddingModel {
    fn to_string(&self) -> String {
        "text-embedding-ada-002".to_string()
    }
}

pub enum OpenAIGenerateModel {
    Gpt35,
    Gpt4,
}

impl ToString for OpenAIGenerateModel {
    fn to_string(&self) -> String {
        match self {
            OpenAIGenerateModel::Gpt35 => "gpt-3.5-turbo".to_string(),
            OpenAIGenerateModel::Gpt4 => "gpt-4".to_string(),
        }
    }
}

pub struct OpenAI {
    api_base: RwLock<String>,
    api_key: String,

    // The maximum number of tokens allowed for the generated answer.
    // By default, the number of tokens the model can return will be (4095 - prompt tokens).
    max_tokens: RwLock<u16>,

    // What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.
    // We generally recommend altering this or top_p but not both.
    temperature: RwLock<f32>,

    embedding_model: RwLock<OpenAIEmbeddingModel>,
    generate_model: RwLock<OpenAIGenerateModel>,
}

impl OpenAI {
    pub fn create(api_key: &str) -> Arc<Self> {
        Arc::new(OpenAI {
            api_base: RwLock::new("https://api.openai.com/v1/".to_string()),
            api_key: api_key.to_string(),
            max_tokens: RwLock::new(4095),
            temperature: RwLock::new(1.0),
            embedding_model: RwLock::new(OpenAIEmbeddingModel::TextEmbeddingAda002),
            generate_model: RwLock::new(OpenAIGenerateModel::Gpt35),
        })
    }

    pub fn with_api_base<S: Into<String>>(self: &Arc<Self>, api_base: S) -> Arc<Self> {
        *self.api_base.write() = api_base.into();
        self.clone()
    }

    pub fn with_max_tokens(self: &Arc<Self>, max_tokens: u16) -> Arc<Self> {
        *self.max_tokens.write() = max_tokens;
        self.clone()
    }

    pub fn with_embedding_model(self: &Arc<Self>, model: OpenAIEmbeddingModel) -> Arc<Self> {
        *self.embedding_model.write() = model;
        self.clone()
    }

    pub fn with_generate_model(self: &Arc<Self>, model: OpenAIGenerateModel) -> Arc<Self> {
        *self.generate_model.write() = model;
        self.clone()
    }

    pub fn with_temperature(self: &Arc<Self>, temperature: f32) -> Arc<Self> {
        *self.temperature.write() = temperature;
        self.clone()
    }

    pub fn get_client(&self) -> Client<OpenAIConfig> {
        let conf = OpenAIConfig::new()
            .with_api_key(&self.api_key)
            .with_api_base(&*self.api_base.read());
        Client::new(conf)
    }
}

#[async_trait::async_trait]
impl LLM for OpenAI {
    async fn embedding(&self, inputs: Vec<String>) -> Result<EmbeddingResult> {
        let request = CreateEmbeddingRequestArgs::default()
            .model(&*self.embedding_model.read().to_string())
            .input(inputs)
            .build()?;

        let client = self.get_client();
        let response = client.embeddings().create(request).await?;
        let mut embeddings = Vec::with_capacity(response.data.len());
        for embedding in &response.data {
            embeddings.push(embedding.embedding.clone());
        }

        let embedding_result = EmbeddingResult {
            prompt_tokens: response.usage.prompt_tokens,
            total_tokens: response.usage.total_tokens,
            embeddings,
        };
        Ok(embedding_result)
    }

    async fn generate(&self, input: &str) -> Result<GenerateResult> {
        let request = CreateChatCompletionRequestArgs::default()
            .max_tokens(*self.max_tokens.read() - input.len() as u16)
            .model(&*self.generate_model.read().to_string())
            .temperature(*self.temperature.read())
            .messages([ChatCompletionRequestMessageArgs::default()
                .role(Role::Assistant)
                .content(input)
                .build()?])
            .build()?;

        let client = self.get_client();
        let response = client.chat().create(request).await?;

        let mut generate_result = GenerateResult::default();

        // Usage.
        if let Some(usage) = response.usage {
            generate_result.prompt_tokens = usage.prompt_tokens;
            generate_result.total_tokens = usage.total_tokens;
            generate_result.completion_tokens = usage.completion_tokens;
        }

        if !response.choices.is_empty() {
            generate_result.generation = response.choices[0].message.content.clone();
        }

        Ok(generate_result)
    }
}
