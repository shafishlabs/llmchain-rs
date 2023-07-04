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

use crate::chat_tokens;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Document {
    pub path: String,
    pub content: String,
    pub content_md5: String,
}

impl Document {
    pub fn create(path: &str, content: &str) -> Self {
        Document {
            path: path.to_string(),
            content: content.to_string(),
            content_md5: format!("{:x}", md5::compute(content)),
        }
    }

    pub fn tokens(&self) -> usize {
        chat_tokens(&self.content).unwrap().len()
    }

    pub fn size(&self) -> usize {
        self.content.len()
    }
}
