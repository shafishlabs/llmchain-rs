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

use anyhow::Result;
use parking_lot::RwLock;

use crate::Prompt;
use crate::PromptTemplate;

pub struct DocumentRetrievalPrompt {
    instructions: RwLock<Vec<String>>,
}

impl DocumentRetrievalPrompt {
    pub fn create() -> Self {
        DocumentRetrievalPrompt {
            instructions: RwLock::new(Vec::new()),
        }
    }

    pub fn with_instructions(self, instructions: Vec<&str>) -> Self {
        let instructs: Vec<_> = instructions.into_iter().map(|s| s.to_string()).collect();
        self.instructions.write().extend(instructs);
        self
    }
}

impl Prompt for DocumentRetrievalPrompt {
    // https://github.com/jerryjliu/llama_index/blob/main/llama_index/prompts/default_prompts.py
    // https://github.com/hwchase17/langchain/blob/master/langchain/chains/qa_with_sources/stuff_prompt.py
    fn template(&self) -> String {
        // Contexts format as:
        // Content: xx...
        // Source: 0-pl
        // Content: yy
        // Source: 24-pl
        r#"Given the following contexts of a long document and a question, create a final answer with references (\"SOURCES\").
            If you don't know the answer, just say that you don't know. Don't try to make up an answer.
            please follow these instructions
            {instructions}
            =========
            {contexts}
            =========
            QUESTION: {question}
            FINAL ANSWER:"#.to_string()
    }

    fn variables(&self) -> Vec<String> {
        vec!["contexts".to_string(), "question".to_string()]
    }

    fn format(&self, input_variables: HashMap<&str, &str>) -> Result<String> {
        // replace instructions.
        let instructions = self.instructions.read().join(" \n");
        let prompt_template = self.template().replace("{instructions}", &instructions);

        let prompt_template = PromptTemplate::create(&prompt_template, self.variables());
        prompt_template.format(input_variables)
    }
}
