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
        r#"You are an expert programmer summarizing code changes, please provide a clear and concise summary of the main changes made in a pull request. Focus on the motivation behind the changes and avoid describing specific file modifications. Follow these guidelines while summarizing:
            1. Ignore changes that you think are not important.
            2. Summarize and classify all changelogs into 1 to 5 points.
            3. Remove the similar points.
            4. Summarize a title for each point, format is `* **Title**`, describing what the point mainly did, as a new title for the pull request changelog, no more than 30 words.
            5. Make an understandable summary for each point with in 50 words, mainly for the background of this change.
            --------
            {text}"#.to_string()
    }

    fn variables(&self) -> Vec<String> {
        vec!["text".to_string()]
    }

    fn format(&self, input_variables: HashMap<&str, &str>) -> Result<String> {
        let prompt_template = PromptTemplate::create(&self.template(), self.variables());
        prompt_template.format(input_variables)
    }
}
