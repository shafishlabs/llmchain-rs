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

use crate::Disk;
use crate::Document;
use crate::DocumentLoader;
use crate::DocumentPath;
use crate::Documents;

pub struct TextLoader {
    disk: Arc<dyn Disk>,
}

impl TextLoader {
    pub fn create(disk: Arc<dyn Disk>) -> Arc<Self> {
        Arc::new(TextLoader { disk })
    }
}

#[async_trait::async_trait]
impl DocumentLoader for TextLoader {
    async fn load(&self, path: DocumentPath) -> Result<Documents> {
        let bs = self.disk.get_operator()?.read(path.as_str()?).await?;
        let content = String::from_utf8_lossy(&bs).to_string();
        let documents = Documents::create();
        documents.push(Document::create(path.as_str()?, &content));

        Ok(documents)
    }
}
