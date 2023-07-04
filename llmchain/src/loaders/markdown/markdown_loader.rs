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
use crate::DocumentLoader;
use crate::DocumentPath;
use crate::Documents;
use crate::TextLoader;

pub struct MarkdownLoader {
    disk: Arc<dyn Disk>,
}

impl MarkdownLoader {
    pub fn create(disk: Arc<dyn Disk>) -> Arc<Self> {
        Arc::new(MarkdownLoader { disk })
    }
}

#[async_trait::async_trait]
impl DocumentLoader for MarkdownLoader {
    async fn load(&self, path: DocumentPath) -> Result<Documents> {
        let text_loader = TextLoader::create(self.disk.clone());
        text_loader.load(path).await
    }
}
