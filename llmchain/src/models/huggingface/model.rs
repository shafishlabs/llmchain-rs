// Copyright 2024 Shafish Labs.
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
use hf_hub::api::sync::Api;
use hf_hub::{Repo, RepoType};
use crate::models::huggingface::ModelType;
use crate::models::model::Model;

pub struct HuggingFaceModel {
    pub model_type: ModelType,
}

impl HuggingFaceModel {
    pub fn new(model_type: ModelType) -> Self {
        Self {
            model_type,
        }
    }
}


#[async_trait::async_trait]
impl Model for HuggingFaceModel {
    async fn pull(&self) -> Result<()> {
        let revision = "main".to_string();
        let api = Api::new()?;
        let repo = self.model_type.repo();
        let api = api.repo(Repo::with_revision(repo.repo_name, RepoType::Model, revision));
    }
}