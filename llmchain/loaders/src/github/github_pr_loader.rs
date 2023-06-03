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

use std::sync::Arc;

use anyhow::Result;

use crate::Document;
use crate::DocumentLoader;
use crate::DocumentPath;

pub struct GithubPRLoader {
    owner: String,
    repo: String,
}

impl GithubPRLoader {
    pub fn create(owner: &str, repo: &str) -> Arc<Self> {
        Arc::new(GithubPRLoader {
            owner: owner.to_string(),
            repo: repo.to_string(),
        })
    }
}

impl DocumentLoader for GithubPRLoader {
    fn load(&self, _path: DocumentPath) -> Result<Vec<Document>> {
        let _ = self.repo;
        let _ = self.owner;
        todo!()
    }
}
