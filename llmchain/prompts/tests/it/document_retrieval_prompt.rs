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
use std::io::Write;

use anyhow::Result;
use goldenfile::Mint;
use llmchain_prompts::DocumentRetrievalPrompt;
use llmchain_prompts::Prompt;

#[test]
fn test_prompt_document_retrieval() -> Result<()> {
    // testdata dir.
    let curdir = std::env::current_dir()?.to_str().unwrap().to_string();
    let testdata_dir = format!("{}/tests/testdata", curdir);

    let mut mint = Mint::new(testdata_dir);
    let golden_path = "document_retrieval_prompt.golden";
    let mut file = mint.new_goldenfile(golden_path)?;

    let prompt_template = DocumentRetrievalPrompt::create().with_instructions(vec!["Present your answer in markdown format, including code snippets if have, format the code snippets with SQL type if necessary.",
                                               "Do not include any links or external references in your response.\n",
                                               "Do not change the code snippets.\n",
                                               "Do not change the SQL syntax, please don't make up the function.\n",
                                               "Do not change explain any code snippets.\n",
                                               "Make the whole answer as short as possible to keep the code snippets.\n"
    ]);

    // invalid input variable.
    {
        let mut input_variables = HashMap::new();
        input_variables.insert("1", "v");
        let result = prompt_template.format(input_variables);
        assert!(result.is_err());
    }

    // ok.
    {
        let mut input_variables = HashMap::new();
        input_variables.insert(
            "contexts",
            "Content: Welcome to the Databend documentation! Databend is an open-source, elastic, and workload-aware modern cloud data warehouse designed to meet businesses' massive-scale analytics needs at low cost and with low complexity.\nSource:1.md\nConent: Databend is always searching for and incorporating the most advanced and innovative technologies to provide you with an exceptional user experience.\nSource:2.md",
        );
        input_variables.insert("question", "what is databend");
        let result = prompt_template.format(input_variables)?;

        writeln!(file, "------------------")?;
        writeln!(file, "{:?}", result)?;
        writeln!(file, "------------------")?;
    }

    Ok(())
}
