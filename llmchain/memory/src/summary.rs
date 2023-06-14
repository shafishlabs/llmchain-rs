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
use llmchain_prompts::Prompt;
use llmchain_prompts::PromptTemplate;

pub struct Summary {
    llm: Arc<dyn LLM>,
    summaries: Vec<String>,
}

impl Summary {
    pub fn create(llm: Arc<dyn LLM>) -> Self {
        Self {
            llm,
            summaries: vec![],
        }
    }

    pub async fn add(&mut self, value: &str) -> Result<()> {
        let prompt_template =
            PromptTemplate::create("Summarize the following text: {{text}}", vec![
                "text".to_string(),
            ]);
        let mut input_variables = HashMap::new();
        input_variables.insert("text", value);
        let prompt = prompt_template.format(input_variables)?;

        let summary = self.llm.generate(&prompt).await?;

        self.summaries.push(summary.generation);

        Ok(())
    }

    pub async fn summary(&self) -> Result<String> {
        let instruction =
            " Format the following text into a Pull Request Body with the following sections:
             - Summary
             - List of changes
             - Refactoring Target
             Remove duplicate information.

            Text to format: ";

        let contents = self.summaries.join("----\n");

        let summary = self
            .llm
            .generate(&(instruction.to_string() + &*contents))
            .await?;

        Ok(summary.generation)
    }
}
