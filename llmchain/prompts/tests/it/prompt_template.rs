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
use llmchain_prompts::Prompt;
use llmchain_prompts::PromptTemplate;

#[test]
fn test_prompt_template() -> Result<()> {
    // testdata dir.
    let curdir = std::env::current_dir()?.to_str().unwrap().to_string();
    let testdata_dir = format!("{}/tests/testdata", curdir);

    let mut mint = Mint::new(testdata_dir);
    let golden_path = "prompt_template.golden";
    let mut file = mint.new_goldenfile(golden_path)?;

    let prompt_template = PromptTemplate::create("hello {{name}}", vec!["name".to_string()]);

    // invalid input variable.
    {
        let mut input_variables = HashMap::new();
        input_variables.insert("1", "v");
        let result = prompt_template.format(input_variables);

        writeln!(file, "------------------")?;
        writeln!(file, "{:?}", result.err())?;
        writeln!(file, "------------------")?;
    }

    // ok.
    {
        let mut input_variables = HashMap::new();
        input_variables.insert("name", "llmchain.rs");
        let result = prompt_template.format(input_variables)?;

        writeln!(file, "------------------")?;
        writeln!(file, "{:?}", result)?;
        writeln!(file, "------------------")?;
    }

    Ok(())
}
