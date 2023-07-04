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
use log::info;
use octocrab::Octocrab;

use crate::chat_tokens;
use crate::Document;
use crate::DocumentLoader;
use crate::DocumentPath;
use crate::Documents;
pub struct GithubPRLoader {
    owner: String,
    repo: String,
    person_token: String,
}

impl GithubPRLoader {
    pub fn create(owner: &str, repo: &str, person_token: &str) -> Arc<Self> {
        Arc::new(GithubPRLoader {
            owner: owner.to_string(),
            repo: repo.to_string(),
            person_token: person_token.to_string(),
        })
    }
}

#[async_trait::async_trait]
impl DocumentLoader for GithubPRLoader {
    async fn load(&self, path: DocumentPath) -> Result<Documents> {
        let documents = Documents::create();
        let list = path.as_list()?;
        info!("Loading PRs from {:?}", list);

        for id in list {
            let now = std::time::Instant::now();
            let octocrab = octocrab::initialise(
                Octocrab::builder()
                    .personal_token(self.person_token.clone())
                    .build()?,
            );
            let diff = octocrab
                .pulls(&self.owner, &self.repo)
                .get_diff(id as u64)
                .await;

            let path = format!(
                "https://github.com/{}/{}/pull/{}",
                self.owner, self.repo, id
            );
            if diff.is_err() {
                info!("PR {} not found, error:{:?}", path, diff.err());
                continue;
            }

            let diff = diff?;
            documents.push(Document::create(&path, &diff));
            info!(
                "Loaded PR {}, diff_len {}, tokens {} in {:?}",
                path,
                diff.len(),
                chat_tokens(&diff).unwrap().len(),
                now.elapsed()
            );
        }

        Ok(documents)
    }
}
