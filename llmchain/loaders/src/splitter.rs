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
use regex::Regex;

use crate::document::Document;
use crate::document::DocumentSettings;

pub trait TextSplitter {
    fn separators(&self) -> Vec<String>;

    fn settings(&self) -> DocumentSettings;

    fn split_text(&self, text: &str) -> Result<Vec<String>> {
        // Splits.
        let separators = self.separators();
        let separator_pattern = separators
            .iter()
            .map(|separator| regex::escape(separator))
            .collect::<Vec<String>>()
            .join("|");
        let separator_regex = Regex::new(&separator_pattern)?;

        let mut parts = Vec::new();
        let mut last_end = 0;
        for cap in separator_regex.find_iter(text) {
            let part = &text[last_end..cap.start()];
            last_end = cap.end();
            parts.push(part.to_string());
        }
        parts.push(text[last_end..].to_string());

        // Merge.
        let settings = self.settings();
        let mut docs = Vec::new();
        let mut current_chunk = String::new();
        for part in &parts {
            if current_chunk.len() > settings.splitter_chunk_size {
                docs.push(current_chunk.clone());
                current_chunk.clear();
            } else if current_chunk.len() + part.len() >= settings.splitter_chunk_size {
                current_chunk.push(' ');
                current_chunk.push_str(part);
                docs.push(current_chunk.clone());
                current_chunk.clear();
            } else {
                if !current_chunk.is_empty() {
                    current_chunk.push(' ');
                }
                current_chunk.push_str(part);
            }
        }

        if !current_chunk.is_empty() {
            docs.push(current_chunk);
        }

        Ok(docs)
    }

    fn split_documents(&self, documents: &[Document]) -> Result<Vec<Document>> {
        let mut result = vec![];

        for document in documents {
            let meta = document.meta.clone();
            let chunks = self.split_text(&document.content)?;

            for chunk in chunks {
                result.push(Document {
                    meta: meta.clone(),
                    content: chunk,
                })
            }
        }
        Ok(result)
    }
}
