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

use crate::document_splitter::DocumentSplitterSettings;
use crate::document_splitter::TextSplitter;

pub struct MarkdownSplitter {
    settings: DocumentSplitterSettings,
}

impl MarkdownSplitter {
    pub fn create() -> Self {
        MarkdownSplitter {
            settings: DocumentSplitterSettings {
                splitter_chunk_size: 400,
            },
        }
    }

    pub fn with_chunk_size(mut self, chunk_size: usize) -> Self {
        self.settings.splitter_chunk_size = chunk_size;
        self
    }
}

impl TextSplitter for MarkdownSplitter {
    fn separators(&self) -> Vec<String> {
        vec![
            "\n## ".to_string(),
            "\n### ".to_string(),
            "\n#### ".to_string(),
            "\n##### ".to_string(),
            "\n###### ".to_string(),
        ]
    }

    fn settings(&self) -> DocumentSplitterSettings {
        self.settings.clone()
    }
}
