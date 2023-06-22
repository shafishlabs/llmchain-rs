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

use crate::Prompt;
use crate::PromptTemplate;

pub struct GithubPRSummaryPrompt {}

impl GithubPRSummaryPrompt {
    pub fn create() -> Self {
        GithubPRSummaryPrompt {}
    }
}

impl Prompt for GithubPRSummaryPrompt {
    fn template(&self) -> String {
        let tpl = vec![
            "Please follow these instructions to create a concise GitHub pull request summary based on the provided changelogs",
            "using the following format for each key change:",
            "1. Identify and summarize the key changes in 1 to 4 main points.",
            "2. For each point, write a title of 10 words or less.",
            "3. Provide a brief description of each point, using 60 words or less.",
            "4. Do not additional summary at the end of the output.",
            "The output format example for your reference:",
            "```
## Summary
* **Efficient table deletion**

The code now supports deleting all rows in a table more efficiently.

* **Improved readability**

Added comments throughout the codebase to enhance user understanding.
```",
            "changelogs:",
            "{text}",
        ];
        tpl.join("\n")
    }

    fn variables(&self) -> Vec<String> {
        vec!["text".to_string()]
    }

    fn format(&self, input_variables: HashMap<&str, &str>) -> Result<String> {
        let prompt_template = PromptTemplate::create(&self.template(), self.variables());
        prompt_template.format(input_variables)
    }
}
