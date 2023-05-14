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
use opendal::BlockingOperator;

use crate::Document;
use crate::DocumentLoader;
use crate::DocumentMeta;

pub struct TextLoader {
    op: BlockingOperator,
}

impl TextLoader {
    pub fn create(op: BlockingOperator) -> Arc<Self> {
        Arc::new(TextLoader { op })
    }
}

impl DocumentLoader for TextLoader {
    fn load(&self, path: &str) -> Result<Vec<Document>> {
        let bs = self.op.read(path)?;
        let content = String::from_utf8_lossy(&bs).to_string();

        Ok(vec![Document {
            meta: DocumentMeta {
                path: path.to_string(),
            },
            content,
        }])
    }
}
