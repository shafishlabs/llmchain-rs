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

use anyhow::anyhow;
use anyhow::Result;

pub trait Prompt: Send + Sync {
    fn template(&self) -> String;
    fn variables(&self) -> Vec<String>;
    fn format(&self, input_variables: HashMap<&str, &str>) -> Result<String>;
}

pub struct PromptTemplate {
    template: String,
    variables: Vec<String>,
}

impl PromptTemplate {
    pub fn create(template: &str, variables: Vec<String>) -> Arc<PromptTemplate> {
        Arc::new(PromptTemplate {
            template: template.to_string(),
            variables,
        })
    }
}

impl Prompt for PromptTemplate {
    fn template(&self) -> String {
        self.template.clone()
    }

    fn variables(&self) -> Vec<String> {
        self.variables.clone()
    }

    fn format(&self, input_variables: HashMap<&str, &str>) -> Result<String> {
        let mut prompt = self.template();

        // Check.
        for (key, value) in input_variables {
            if !self.variables().contains(&key.to_string()) {
                return Err(anyhow!(
                    "input variable: '{}' is not in the variables: {:?}",
                    key,
                    self.variables()
                ));
            }

            let key = format!("{{{}}}", key);
            prompt = prompt.replace(&key, value);
        }

        Ok(prompt)
    }
}
