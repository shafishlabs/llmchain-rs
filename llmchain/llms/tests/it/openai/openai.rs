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
use llmchain_llms::OpenAI;
use llmchain_llms::OpenAIGenerateModel;
use llmchain_llms::LLM;

#[ignore]
#[tokio::test]
async fn test_llm_openai_generate_gpt35() -> Result<()> {
    let api_key = std::env::var("OPENAI_API_KEY").unwrap_or("".to_string());

    let llm = OpenAI::create(&api_key);
    let result = llm.generate("say Hello").await?;
    let generation = result.generation;
    assert!(generation.contains("Hello"));
    assert_eq!(result.prompt_tokens, 10);
    assert_eq!(result.completion_tokens, 9);
    assert_eq!(result.total_tokens, 19);

    Ok(())
}

#[ignore]
#[tokio::test]
async fn test_llm_openai_generate_gpt4() -> Result<()> {
    let api_key = std::env::var("OPENAI_API_KEY").unwrap_or("".to_string());

    let llm = OpenAI::create(&api_key).with_generate_model(OpenAIGenerateModel::Gpt4);
    let result = llm.generate("say Hello").await?;
    let generation = result.generation;
    assert!(generation.contains("Hello"));
    assert_eq!(result.prompt_tokens, 9);
    assert_eq!(result.completion_tokens, 2);
    assert_eq!(result.total_tokens, 11);

    Ok(())
}

#[ignore]
#[tokio::test]
async fn test_llm_openai_embedding() -> Result<()> {
    let api_key = std::env::var("OPENAI_API_KEY").unwrap_or("".to_string());
    let llm = OpenAI::create(&api_key);
    let inputs = vec!["llmchain".to_string(), "rs".to_string()];
    let result = llm.embedding(inputs).await?;
    let embeddings = result.embeddings;
    assert_eq!(embeddings.len(), 2);

    assert_eq!(embeddings[0].len(), 1536);
    assert_eq!(embeddings[1].len(), 1536);
    assert_eq!(result.prompt_tokens, 4);
    assert_eq!(result.total_tokens, 4);

    Ok(())
}
