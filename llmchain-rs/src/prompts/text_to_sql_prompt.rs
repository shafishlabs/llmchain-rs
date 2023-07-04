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

pub struct TextToSQLPrompt {}

impl TextToSQLPrompt {
    pub fn create() -> Self {
        TextToSQLPrompt {}
    }
}

impl Prompt for TextToSQLPrompt {
    fn template(&self) -> String {
        let tpl = vec![
            "Given an input question, first create a syntactically correct {dialect} ",
            "query to run, then look at the results of the query and return the answer. ",
            "You can order the results by a relevant column to return the most ",
            "interesting examples in the database.\n",
            "Never query for all the columns from a specific table, only ask for a the ",
            "few relevant columns given the question.\n",
            "Pay attention to use only the column names that you can see in the schema ",
            "description. ",
            "Be careful to not query for columns that do not exist. ",
            "Pay attention to which column is in which table. ",
            "Also, qualify column names with the table name when needed.\n",
            "Use the following format:\n",
            "Question: Question here\n",
            "SQLQuery: SQL Query to run\n",
            "SQLResult: Result of the SQLQuery\n",
            "Answer: Final answer here\n",
            "Only use the tables listed below.\n",
            "{schema}\n",
            "Question: {query_str}\n",
            "SQLQuery: ",
        ];
        tpl.join("")
    }

    fn variables(&self) -> Vec<String> {
        vec![
            "dialect".to_string(),
            "schema".to_string(),
            "query_str".to_string(),
        ]
    }

    fn format(&self, input_variables: HashMap<&str, &str>) -> Result<String> {
        let prompt_template = PromptTemplate::create(&self.template(), self.variables());
        prompt_template.format(input_variables)
    }
}
