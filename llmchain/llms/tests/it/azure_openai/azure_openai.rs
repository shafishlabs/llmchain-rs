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
use llmchain_llms::AzureOpenAI;
use llmchain_llms::LLM;

#[tokio::test]
async fn test_llm_azure_openai_generate_gpt35() -> Result<()> {
    let api_base = std::env::var("AZURE_OPENAI_API_BASE").unwrap_or("".to_string());
    let api_key = std::env::var("AZURE_OPENAI_API_KEY").unwrap_or("".to_string());
    let api_deployment = std::env::var("AZURE_OPENAI_API_GEN_DEPLOYMENT").unwrap_or("".to_string());

    let llm = AzureOpenAI::create(&api_base, &api_key, &api_deployment);
    let result = llm.generate("say Hello").await?;
    let generation = result.generation;
    assert!(generation.contains("Hello"));
    assert_eq!(result.prompt_tokens, 10);
    assert_eq!(result.completion_tokens, 9);
    assert_eq!(result.total_tokens, 19);

    Ok(())
}

#[tokio::test]
async fn test_llm_azure_openai_embedding() -> Result<()> {
    let api_base = std::env::var("AZURE_OPENAI_API_BASE").unwrap_or("".to_string());
    let api_key = std::env::var("AZURE_OPENAI_API_KEY").unwrap_or("".to_string());
    let api_deployment =
        std::env::var("AZURE_OPENAI_API_EMBED_DEPLOYMENT").unwrap_or("".to_string());

    let llm = AzureOpenAI::create(&api_base, &api_key, &api_deployment);

    let inputs = vec!["llmchain".to_string()];
    let result = llm.embedding(inputs).await?;
    let embeddings = result.embeddings;
    assert_eq!(embeddings.len(), 1);

    assert_eq!(embeddings[0].len(), 1536);
    assert_eq!(result.prompt_tokens, 3);
    assert_eq!(result.total_tokens, 3);

    Ok(())
}
