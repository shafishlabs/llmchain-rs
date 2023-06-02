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
use llmchain_llms::DatabendLLM;
use llmchain_llms::LLM;

#[tokio::test]
async fn test_llm_databend_embedding() -> Result<()> {
    let dsn = std::env::var("DATABEND_DSN").unwrap();

    let llm = DatabendLLM::create(&dsn);

    let inputs = vec!["llmchain".to_string(), "rs".to_string()];
    let result = llm.embedding(inputs).await?;
    let embeddings = result.embeddings;
    assert_eq!(embeddings.len(), 2);

    assert_eq!(embeddings[0].len(), 1536);
    assert_eq!(embeddings[1].len(), 1536);
    assert_eq!(result.prompt_tokens, 0);
    assert_eq!(result.total_tokens, 0);

    Ok(())
}
