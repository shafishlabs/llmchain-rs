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
use crate::DocumentLoader;
use crate::DocumentPath;
use crate::Documents;
use crate::LocalDisk;
use crate::MarkdownLoader;
use crate::TextLoader;

pub struct GithubRepoLoader {}

impl GithubRepoLoader {
    pub fn create() -> Arc<Self> {
        Arc::new(GithubRepoLoader {})
    }
}

#[async_trait::async_trait]
impl DocumentLoader for GithubRepoLoader {
    async fn load(&self, path: DocumentPath) -> Result<Documents> {
        let repo_url = path.as_str()?;
        let local_path = format!("/tmp/{}/", uuid::Uuid::new_v4());
        let local_disk = LocalDisk::create()?;

        {
            local_disk.get_operator()?.remove_all(&local_path).await?;
            info!("remove {}", local_path);
        }

        {
            info!("Cloning {} to {}", repo_url, local_path);
            let _ = Repository::clone(repo_url, &local_path)?;
        }

        let directory = DirectoryLoader::create(local_disk.clone())
            .with_loader("**/*.rs", TextLoader::create(local_disk.clone()))
            .with_loader("**/*.md", MarkdownLoader::create(local_disk.clone()));

        let result = directory
            .load(DocumentPath::Str(local_path.clone()))
            .await?;
        info!("DirectoryLoader result: {:?}", result.len());

        let result = result
            .iter()
            .map(|x| {
                let mut x = x;
                x.path = x.path.replace(&local_path, repo_url);
                x
            })
            .collect::<Vec<_>>();

        {
            local_disk.get_operator()?.remove_all(&local_path).await?;
            info!("remove {}", local_path);
        }

        Ok(Documents::from(result))
    }
}
