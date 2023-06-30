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
use env_logger::Env;
use llmchain::Prompt;
use llmchain::PromptTemplate;
use log::info;

/// cargo run --bin example_prompt_template
#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // prompt template.
    let prompt_template = PromptTemplate::create("Hello {{name}}", vec!["name".to_string()]);

    // input variables.
    let mut input_variables = HashMap::new();
    input_variables.insert("name", "llmchain.rs");

    // format the template.
    let prompt = prompt_template.format(input_variables)?;

    info!("prompt: {}", prompt);

    Ok(())
}
