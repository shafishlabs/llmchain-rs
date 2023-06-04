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

use crate::text::TextSplitter;
use crate::Document;
use crate::DocumentSplitter;

pub struct PdfSplitter {
    pub splitter_chunk_size: usize,
}

impl PdfSplitter {
    pub fn create() -> Self {
        PdfSplitter {
            splitter_chunk_size: 400,
        }
    }

    pub fn with_chunk_size(mut self, chunk_size: usize) -> Self {
        self.splitter_chunk_size = chunk_size;
        self
    }
}

impl DocumentSplitter for PdfSplitter {
    fn separators(&self) -> Vec<String> {
        vec![
        ]
    }

    fn split_documents(&self, documents: &[Document]) -> Result<Vec<Document>> {
        let text_splitter = TextSplitter::create()
            .with_chunk_size(self.splitter_chunk_size)
            .with_separators(self.separators());
        text_splitter.split_documents(documents)
    }
}
