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

pub struct Markdown {
    pub op: Operator,
}

impl Markdown {
    pub fn create(op: Operator) -> Self {
        Markdown { op }
    }
}

#[async_trait::async_trait]
impl DocumentLoader for Markdown {
    async fn load(&self, path: &str) -> Result<Vec<Document>> {
        let bs = self.op.read(path).await?;
        let content = String::from_utf8_lossy(&bs).to_string();

        Ok(vec![Document {
            meta: DocumentMeta {
                path: path.to_string(),
            },
            content,
        }])
    }
}
