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

use std::ffi::OsString;

use anyhow::Result;
use clap::Parser;
use llmchain_llms::openai::OpenAIConfig;

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_llm_openai_config() -> Result<()> {
    let expect = OpenAIConfig::default();
    let actual: OpenAIConfig = OpenAIConfig::parse_from(Vec::<OsString>::new());

    assert_eq!(expect.generate_model, actual.generate_model);
    assert_eq!(expect.embedding_model, actual.embedding_model);
    assert_eq!(expect.max_token, actual.max_token);
    assert_eq!(expect.temperature, actual.temperature);
    Ok(())
}
