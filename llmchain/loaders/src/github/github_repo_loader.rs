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
use git2::Repository;
use log::info;

use crate::DirectoryLoader;
use crate::Disk;
use crate::Document;
use crate::DocumentLoader;
use crate::DocumentPath;
use crate::LocalDisk;
use crate::MarkdownLoader;
use crate::TextLoader;

pub struct GithubRepoLoader {
    owner: String,
    repo: String,
    domain: String,
}

impl GithubRepoLoader {
    pub fn create(owner: &str, repo: &str, _person_token: &str) -> Arc<Self> {
        Arc::new(GithubRepoLoader {
            owner: owner.to_string(),
            repo: repo.to_string(),
            domain: "github.com".to_string(),
        })
    }
}

#[async_trait::async_trait]
impl DocumentLoader for GithubRepoLoader {
    async fn load(&self, _path: DocumentPath) -> Result<Vec<Document>> {
        let prefix_path = format!("/tmp/{}/", uuid::Uuid::new_v4());
        let repo_clone_path = format!(
            "{}{}/{}/{}/",
            prefix_path, self.domain, self.owner, self.repo
        );
        let local_disk = LocalDisk::create()?;

        {
            local_disk
                .get_operator()?
                .remove_all(&repo_clone_path)
                .await?;
            info!("remove {}", repo_clone_path);
        }

        let url = format!("https://{}/{}/{}", self.domain, self.owner, self.repo);

        {
            info!("Cloning {} to {}", url, repo_clone_path);
            let _ = Repository::clone(&url, &repo_clone_path)?;
        }

        let directory = DirectoryLoader::create(local_disk.clone())
            .with_loader("**/*.rs", TextLoader::create(local_disk.clone()))
            .with_loader("**/*.md", MarkdownLoader::create(local_disk.clone()));

        let result = directory
            .load(DocumentPath::Str(repo_clone_path.clone()))
            .await?;
        info!("DirectoryLoader result: {:?}", result.len());

        let result = result
            .iter()
            .map(|x| {
                let mut x = x.clone();
                x.path = x.path.replace(&prefix_path, "https://");
                x
            })
            .collect::<Vec<_>>();

        {
            local_disk
                .get_operator()?
                .remove_all(&repo_clone_path)
                .await?;
            info!("remove {}", repo_clone_path);
        }

        Ok(result)
    }
}
