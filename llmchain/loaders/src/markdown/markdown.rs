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
use opendal::Operator;

use crate::document::Document;
use crate::document::DocumentLoader;
use crate::document::DocumentMeta;
use crate::document::DocumentSettings;
use crate::markdown::markdown_splitter::MarkdownSplitter;
use crate::splitter::TextSplitter;

pub struct Markdown {
    pub op: Operator,
    pub settings: DocumentSettings,
}

impl Markdown {
    pub fn create(op: Operator, settings: &DocumentSettings) -> Self {
        Markdown {
            op,
            settings: settings.clone(),
        }
    }
}

#[async_trait::async_trait]
impl DocumentLoader for Markdown {
    async fn load(&self, path: &str) -> Result<Vec<Document>> {
        let bs = self.op.read(path).await?;
        let text = String::from_utf8_lossy(&bs).to_string();

        let splitter = MarkdownSplitter::create(&self.settings);
        let chunks = splitter.split(&text)?;

        let mut docs = Vec::with_capacity(chunks.len());
        for chunk in &chunks {
            let doc = Document {
                meta: DocumentMeta {
                    path: path.to_string(),
                },
                content: chunk.clone(),
            };
            docs.push(doc);
        }

        Ok(docs)
    }
}
