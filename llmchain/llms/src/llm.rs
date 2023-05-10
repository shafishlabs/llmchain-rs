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

use anyhow::Result;

#[async_trait::async_trait]
pub trait LLM {
    async fn embedding(&self, inputs: Vec<String>) -> Result<Vec<Vec<f32>>>;
    async fn generate<S: Into<String> + Send>(&self, input: S) -> Result<String>;
    async fn chat(&self, _input: Vec<String>) -> Result<Vec<String>> {
        unimplemented!("")
    }
}
