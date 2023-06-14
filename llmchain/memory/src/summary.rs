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
use llmchain_llms::LLM;
use llmchain_loaders::Document;
use llmchain_prompts::Prompt;
use llmchain_prompts::PromptTemplate;
use parking_lot::RwLock;

pub struct Summary {
    llm: Arc<dyn LLM>,
    summaries: RwLock<Vec<String>>,
}

impl Summary {
    pub fn create(llm: Arc<dyn LLM>) -> Arc<Self> {
        Arc::new(Self {
            llm,
            summaries: RwLock::new(Vec::new()),
        })
    }

    pub async fn add_document(&self, document: &Document) -> Result<()> {
        let prompt_template = PromptTemplate::create(
            "Summarize the following text within 100 words: {{text}}",
            vec!["text".to_string()],
        );
        let mut input_variables = HashMap::new();
        input_variables.insert("text", document.content.as_str());
        let prompt = prompt_template.format(input_variables)?;

        let summary = self.llm.generate(&prompt).await?;

        self.summaries.write().push(summary.generation);

        Ok(())
    }

    pub async fn add_documents(&self, documents: &[Document]) -> Result<()> {
        for document in documents {
            self.add_document(document).await?;
        }

        Ok(())
    }

    pub async fn final_summary(&self, prompt: Arc<dyn Prompt>) -> Result<String> {
        let prompts = prompt.format(HashMap::new())?;
        let contents = self.summaries.read().join("----\n");

        let summary = self
            .llm
            .generate(&(prompts.to_string() + &*contents))
            .await?;

        Ok(summary.generation)
    }
}
