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
use std::env;
use std::sync::Arc;
use std::time::Instant;

use anyhow::Result;
use env_logger::Env;
use llmchain::DatabendEmbedding;
use llmchain::DatabendLLM;
use llmchain::DatabendVectorStore;
use llmchain::DirectoryLoader;
use llmchain::DocumentLoader;
use llmchain::DocumentPath;
use llmchain::DocumentRetrievalPrompt;
use llmchain::DocumentSplitter;
use llmchain::LocalDisk;
use llmchain::MarkdownLoader;
use llmchain::MarkdownSplitter;
use llmchain::Prompt;
use llmchain::VectorStore;
use llmchain::LLM;
use log::info;

/// EXPORT DATABEND_DSN=<your-databend-dsn>
/// cargo run --bin example_document_qa <embedding|query>
#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let dsn = std::env::var("DATABEND_DSN")
        .map_err(|_| {
            "DATABEND_DSN is empty, please EXPORT DATABEND_DSN=<your-databend-dsn>".to_string()
        })
        .unwrap();

    let args: Vec<String> = env::args().collect();
    if !args.is_empty() {
        let arg = args.get(1).unwrap();
        match arg.as_str() {
            "embedding" => embeddings(&dsn).await?,
            "query" => query(&dsn).await?,
            _ => {
                info!("cargo run --bin example_document_qa [embedding|query]")
            }
        }
    }

    Ok(())
}

async fn embeddings(databend_dsn: &str) -> Result<()> {
    // dir.
    let curdir = std::env::current_dir()?.to_str().unwrap().to_string();
    let testdata_dir = format!("{}/examples/testdata", curdir);
    let directory_dir = format!("{}/markdowns/", testdata_dir);

    // Embedding.
    {
        let start = Instant::now();
        // Loader.
        info!("Prepare to load all the documents {}", directory_dir);
        let directory_loader = DirectoryLoader::create(LocalDisk::create()?)
            .with_loader("**/*.md", MarkdownLoader::create(LocalDisk::create()?));
        let documents = directory_loader
            .load(DocumentPath::from_string(&directory_dir))
            .await?;
        info!(
            "Load all the documents {} done, cost: {}",
            directory_dir,
            start.elapsed().as_secs()
        );

        // Splitter.
        info!(
            "Prepare to split all the documents, count: {}",
            documents.len()
        );
        let start = Instant::now();
        let documents = MarkdownSplitter::create().split_documents(&documents)?;
        info!(
            "Split all to documents, count: {}, cost: {}",
            documents.len(),
            start.elapsed().as_secs()
        );

        // embedding.
        info!(
            "Prepare to indexing the documents, count: {}",
            documents.len()
        );
        let start = Instant::now();
        let databend_embedding = Arc::new(DatabendEmbedding::create(databend_dsn));
        let databend = DatabendVectorStore::create(databend_dsn, databend_embedding);
        databend.init().await?;

        // indexing.
        let uuids = databend.add_documents(&documents).await?;
        info!(
            "Indexing the documents done, count: {}, cost: {}",
            uuids.len(),
            start.elapsed().as_secs()
        );

        Ok(())
    }
}

async fn query(databend_dsn: &str) -> Result<()> {
    let start = Instant::now();
    let question = "how to do COPY in databend";

    let databend_embedding = Arc::new(DatabendEmbedding::create(databend_dsn));
    let databend = DatabendVectorStore::create(databend_dsn, databend_embedding);
    databend.init().await?;
    let similarities = databend.similarity_search(question, 3).await?;
    info!(
        "query: {}, similarity documents: {:?}, cost: {}",
        question,
        similarities.len(),
        start.elapsed().as_secs()
    );

    let contexts = similarities
        .iter()
        .map(|x| format!("context:{}\nsource:{}", x.content, x.path))
        .collect::<Vec<_>>()
        .join("");
    let prompt_template = DocumentRetrievalPrompt::create().with_instructions(vec!["Present your answer in markdown format, including code snippets if have, format the code snippets with SQL type if necessary.",
                                                                                   "Do not include any links or external references in your response.\n",
                                                                                   "Do not change the code snippets.\n",
                                                                                   "Do not change the SQL syntax, please don't make up the function.\n",
                                                                                   "Do not change explain any code snippets.\n",
                                                                                   "Make the whole answer as short as possible to keep the code snippets.\n"
    ]);
    let mut input_variables = HashMap::new();
    input_variables.insert("question", question);
    input_variables.insert("contexts", &contexts);
    let prompt = prompt_template.format(input_variables)?;

    //
    let databend_llm = DatabendLLM::create(databend_dsn);
    let answer = databend_llm.generate(&prompt).await?;
    info!("question: {}", question);
    info!("answer: {:?}", answer);
    Ok(())
}
