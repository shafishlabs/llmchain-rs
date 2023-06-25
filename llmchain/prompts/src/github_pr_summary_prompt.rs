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
            "Please provide a clear and concise summary of the main changes made in a pull request. Focus on the motivation behind the changes and avoid describing specific file modifications. Follow these guidelines while summarizing:",
            "1. Ignore changes that you think are not important.",
            "2. Remove the similar points.",
            "3. Summarize and classify all changelogs into 1 to 4 points.",
            "4. Summarize a title for each point, describing what the point mainly did, as a new title for the pull request changelog, no more than 30 words.",
            "5. Make an easy-to-understand summary for each point no more than 80 words, please do not describe what was modified.",
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
