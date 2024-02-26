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
use async_openai::config::OpenAIConfig;
use async_openai::types::ChatCompletionRequestMessageArgs;
use async_openai::types::CreateChatCompletionRequestArgs;
use async_openai::types::CreateEmbeddingRequestArgs;
use async_openai::types::Role;
use async_openai::Client;
use derive_builder::Builder;

use crate::EmbeddingResult;
use crate::GenerateResult;
use crate::LLM;

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

#[derive(Builder)]
#[builder(name = "OpenAIBuilder")]
#[builder(derive(Debug))]
pub struct OpenAI {
    api_base: String,
    api_key: String,
    org_id: Option<String>,

    // The maximum number of tokens allowed for the generated answer.
    // By default, the number of tokens the model can return will be (4095 - prompt tokens).
    max_tokens: u16,

    // What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.
    // We generally recommend altering this or top_p but not both.
    temperature: f32,

    embedding_model: String,
    generate_model: String,

    http_client: reqwest::Client,
}

impl OpenAI {
    pub fn create<S: Into<String>>(api_key: S) -> OpenAI {
        OpenAIBuilder::default()
            .api_key(api_key.into())
            .build()
            .unwrap()
    }

    fn get_client(&self) -> Client<OpenAIConfig> {
        let mut conf = OpenAIConfig::new()
            .with_api_key(&self.api_key)
            .with_api_base(&self.api_base);

        if let Some(org_id) = &self.org_id {
            conf = conf.with_org_id(org_id);
        }

        Client::with_config(conf).with_http_client(self.http_client.clone())
    }
}

#[async_trait::async_trait]
impl LLM for OpenAI {
    async fn embedding(&self, inputs: Vec<String>) -> Result<EmbeddingResult> {
        let request = CreateEmbeddingRequestArgs::default()
            .model(&self.embedding_model.to_string())
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
            .max_tokens(self.max_tokens - input.len() as u16)
            .model(self.generate_model.to_string())
            .temperature(self.temperature)
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

        if let Some(choice) = response.choices.first() {
            generate_result.generation = choice.message.content.clone().unwrap_or_default();
        }

        Ok(generate_result)
    }
}
