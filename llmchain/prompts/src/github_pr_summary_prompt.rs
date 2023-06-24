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
            "Please create a GitHub pull request summary in the form of a changelog based on the provided commit explanations. The changelog should:
            1. Summarize and group the key changes in 1 to 4 main points.
            2. For each point, write a title of 30 words or less for to summary what's the point changed.
            3. Provide a description of each point, using 50 words or less, the impact or purpose of this change.
            4. Do not give any no changes explanation.",
            "Format the output as follows(The provided examples are for illustration purposes only and should not be repeated):
* **Title 1** Description 1
* **Title 2** Description 2",
            "--------",
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
