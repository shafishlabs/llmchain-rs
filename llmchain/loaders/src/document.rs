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
use llmchain_common::chat_tokens;
use parking_lot::RwLock;

use crate::DocumentPath;

#[async_trait::async_trait]
pub trait DocumentLoader: Send + Sync {
    async fn load(&self, path: DocumentPath) -> Result<Documents>;
}

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

#[derive(Debug)]
pub struct Documents {
    documents: RwLock<Vec<Document>>,
}

impl Documents {
    pub fn create() -> Self {
        Documents {
            documents: RwLock::new(vec![]),
        }
    }

    pub fn push(&self, document: Document) {
        self.documents.write().push(document);
    }

    pub fn extend(&self, documents: &Documents) {
        self.documents
            .write()
            .extend(documents.documents.read().clone());
    }

    pub fn documents(&self) -> Vec<Document> {
        self.documents.read().clone()
    }

    pub fn tokens(&self) -> usize {
        self.documents.read().iter().map(|d| d.tokens()).sum()
    }

    pub fn size(&self) -> usize {
        self.documents.read().iter().map(|d| d.size()).sum()
    }

    pub fn len(&self) -> usize {
        self.documents.read().len()
    }
    pub fn is_empty(&self) -> bool {
        self.documents.read().is_empty()
    }

    pub fn iter(&self) -> DocumentsIter {
        let guard = self.documents.read().clone();
        DocumentsIter {
            documents: guard,
            index: 0,
        }
    }
}

impl From<Vec<Document>> for Documents {
    fn from(documents: Vec<Document>) -> Self {
        Documents {
            documents: RwLock::new(documents),
        }
    }
}

pub struct DocumentsIter {
    documents: Vec<Document>,
    index: usize,
}

impl Iterator for DocumentsIter {
    type Item = Document;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.documents.len() {
            let result = self.documents[self.index].clone();
            self.index += 1;
            Some(result)
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a Documents {
    type Item = Document;
    type IntoIter = DocumentsIter;

    fn into_iter(self) -> Self::IntoIter {
        DocumentsIter {
            documents: self.documents.read().clone(),
            index: 0,
        }
    }
}
