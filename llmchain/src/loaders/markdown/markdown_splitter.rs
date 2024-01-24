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

use crate::DocumentSplitter;
use crate::Documents;
use crate::TextSplitter;

pub struct MarkdownSplitter {
    pub splitter_chunk_size: usize,
    pub separators: Vec<String>,
}

impl MarkdownSplitter {
    pub fn create() -> Self {
        MarkdownSplitter {
            splitter_chunk_size: 400,
            separators: vec![
                "\n## ".to_string(),
                "\n### ".to_string(),
                "\n#### ".to_string(),
                "\n##### ".to_string(),
                "\n###### ".to_string(),
            ],
        }
    }

    pub fn with_chunk_size(mut self, chunk_size: usize) -> Self {
        self.splitter_chunk_size = chunk_size;
        self
    }

    pub fn with_separators(mut self, separators: Vec<String>) -> Self {
        self.separators = separators;
        self
    }
}

impl DocumentSplitter for MarkdownSplitter {
    fn separators(&self) -> Vec<String> {
        self.separators.clone()
    }

    fn split_documents(&self, documents: &Documents) -> Result<Documents> {
        let text_splitter = TextSplitter::create()
            .with_chunk_size(self.splitter_chunk_size)
            .with_separators(self.separators());
        text_splitter.split_documents(documents)
    }
}
