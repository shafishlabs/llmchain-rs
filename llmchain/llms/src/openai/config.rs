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

#[derive(clap::Parser, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct OpenAIConfig {
    #[clap(long = "base-url", default_value = "https://api.openai.com/v1/")]
    pub openai_api_base: String,

    #[clap(long = "base-url", default_value = "")]
    pub openai_api_key: String,

    #[clap(long = "temperature", default_value = "0.7")]
    pub temperature: f32,

    #[clap(long = "max-token", default_value = "512")]
    pub max_token: usize,
}

impl Default for OpenAIConfig {
    fn default() -> Self {
        OpenAIConfig {
            openai_api_base: "https://api.openai.com/v1/".to_string(),
            openai_api_key: "".to_string(),
            temperature: 0.7,
            max_token: 512,
        }
    }
}
