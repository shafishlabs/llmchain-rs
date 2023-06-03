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

use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Result;
use env_logger::Env;
use llmchain_embeddings::DatabendEmbedding;
use llmchain_llms::DatabendLLM;
use llmchain_llms::LLM;
use llmchain_loaders::DocumentLoader;
use llmchain_loaders::DocumentPath;
use llmchain_loaders::DocumentSplitter;
use llmchain_loaders::GithubPRLoader;
use llmchain_loaders::GithubPRSplitter;
use llmchain_prompts::DocumentRetrievalPrompt;
use llmchain_prompts::Prompt;
use llmchain_vector_stores::DatabendVectorStore;
use llmchain_vector_stores::VectorStore;
use log::info;

/// EXPORT GITHUB_TOKEN=<your-personal-github-token>
/// EXPORT DATABEND_DSN=<your-databend-dsn>
/// cargo run --bin example_github_inspect
///
/// This example shows how to use LLMChain to inspect a GitHub PR.
/// We will use the Databend vector store to store the embeddings.
/// Find the pr is most likely to change the table meta data.
///
/// Output:
/// [2023-06-03T12:51:50Z INFO  llmchain_loaders::github::github_pr_loader] Loading PRs from [11347, 11391, 11435]
/// [2023-06-03T12:51:52Z INFO  llmchain_loaders::github::github_pr_loader] Loaded PR https://github.com/datafuselabs/databend/pull/11347, diff_len 84677 in 1.749269125s
/// ...
/// [2023-06-03T12:53:07Z INFO  llmchain_llms::databend::databend] embedding 63/65,  time: 681.717208ms
/// [2023-06-03T12:53:08Z INFO  llmchain_llms::databend::databend] embedding 64/65,  time: 690.599458ms
/// [2023-06-03T12:53:09Z INFO  llmchain_llms::databend::databend] embedding 65/65,  time: 560.542584ms
/// [2023-06-03T12:53:21Z INFO  example_github_inspect] question:Which code is most likely to change the table meta data?, similarity documents:1
/// [2023-06-03T12:53:24Z INFO  example_github_inspect] answer: GenerateResult { prompt_tokens: 0, completion_tokens: 0, total_tokens: 0, generation: "The code that is most likely to change the table meta data is `impl Binder`. SOURCE: https://github.com/datafuselabs/databend/pull/11391" }
#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let databend_dsn = std::env::var("DATABEND_DSN")
        .map_err(|_| {
            "DATABEND_DSN is empty, please EXPORT DATABEND_DSN=<your-databend-dsn>".to_string()
        })
        .unwrap();

    let github_token = std::env::var("GITHUB_TOKEN")
        .map_err(|_| {
            "GITHUB_TOKEN is empty, please EXPORT GITHUB_TOKEN=<your-personal-github-token>"
                .to_string()
        })
        .unwrap();

    let question = "Which code is most likely to change the table meta data?";

    // create embedding.
    let databend_embedding = Arc::new(DatabendEmbedding::create(&databend_dsn));

    // create databend vector store.
    let databend = DatabendVectorStore::create(&databend_dsn, databend_embedding)
        .with_database("github")
        .with_table("repos");
    databend.init().await?;

    // documents
    let documents = GithubPRLoader::create("datafuselabs", "databend", &github_token)
        .load(DocumentPath::from_list(vec![11347, 11391, 11435]))
        .await?;
    let documents = GithubPRSplitter::create()
        .with_chunk_size(8000)
        .split_documents(&documents)?;

    // add documents to vector store.
    {
        let _ = databend.add_documents(documents).await?;
    }

    // query a similarity document.
    {
        let similarities = databend.similarity_search(question, 1).await?;
        info!(
            "question:{}, similarity documents:{:?}",
            question,
            similarities.len()
        );

        // get the final result.
        let contexts = similarities
            .iter()
            .map(|x| format!("context:{}\nsource:{}", x.path, x.content))
            .collect::<Vec<_>>()
            .join("");
        let prompt_template = DocumentRetrievalPrompt::create().with_instructions(vec!["Present your answer in markdown format, including code snippets if have, format the code snippets with SQL type if necessary.",
                                                                                       "Only give the pr path(SOURCE) to the answer.\n",
        ]);
        let mut input_variables = HashMap::new();
        input_variables.insert("question", question);
        input_variables.insert("contexts", &contexts);
        let prompt = prompt_template.format(input_variables)?;

        // genrate answer.
        let databend_llm = DatabendLLM::create(&databend_dsn);
        let answer = databend_llm.generate(&prompt).await?;
        info!("answer: {:?}", answer);
    }

    Ok(())
}
