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

use crate::Summarize;

pub struct GithubPRSummary {
    llm: Arc<dyn LLM>,
    summaries: RwLock<Vec<String>>,
}
impl GithubPRSummary {
    pub fn create(llm: Arc<dyn LLM>) -> Arc<Self> {
        Arc::new(Self {
            llm,
            summaries: RwLock::new(Vec::new()),
        })
    }
}

#[async_trait::async_trait]
impl Summarize for GithubPRSummary {
    async fn add_document(&self, document: &Document) -> Result<()> {
        let template = "```
        {text}
        ```
Act as a Code Review Helper, please explain the code changes above in github changelogs style: 1.
";
        let prompt_template = PromptTemplate::create(template, vec!["text".to_string()]);
        let mut input_variables = HashMap::new();
        input_variables.insert("text", document.content.as_str());
        let prompt = prompt_template.format(input_variables)?;

        let summary = self.llm.generate(&prompt).await?;
        self.summaries.write().push(summary.generation);

        Ok(())
    }

    async fn add_documents(&self, documents: &[Document]) -> Result<()> {
        for document in documents {
            self.add_document(document).await?;
        }

        Ok(())
    }

    async fn final_summary(&self) -> Result<String> {
        if self.summaries.read().is_empty() {
            return Ok("".to_string());
        }

        let template = "
{text}
\"\"\"

You are a code reviewer expert, the above is some Changelogs from a pull request by code changes.
please summarizing them into a new github pull request body for release in concise way:
- The fewer the parts the better.
- Group the similarity parts into one.
- Only summarize the important parts, each part with a title of 10 words or less and a summary of 20 words or less.

Format like the follow example:
```
## PR Summary

* **Efficient table deletion**
The code now supports deleting all rows in a table more efficiently.
* **Improved readability**
Added comments throughout the codebase to enhance user understanding.
```
             ";

        let prompt_template = PromptTemplate::create(template, vec!["text".to_string()]);
        let mut input_variables = HashMap::new();
        let text = self.summaries.read().join("\n");
        input_variables.insert("text", text.as_str());
        let prompt = prompt_template.format(input_variables)?;
        println!("prompt:\n{}", prompt);

        let summary = self.llm.generate(&prompt).await?;

        Ok(summary.generation)
    }
}
