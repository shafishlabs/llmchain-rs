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

use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::Arc;

use anyhow::Result;
use glob_match::glob_match;
use opendal::BlockingOperator;
use opendal::EntryMode;
use opendal::Metakey;
use rayon::ThreadPoolBuilder;

use crate::document::Document;
use crate::document::DocumentLoader;

pub struct DirectoryLoader {
    op: BlockingOperator,
    loaders: HashMap<String, Arc<dyn DocumentLoader + Send + Sync>>,
}

impl DirectoryLoader {
    pub fn create(op: BlockingOperator) -> Self {
        DirectoryLoader {
            op,
            loaders: HashMap::default(),
        }
    }

    pub fn with_loader(
        mut self,
        glob: &str,
        loader: Arc<dyn DocumentLoader + Send + Sync>,
    ) -> Self {
        self.loaders.insert(glob.to_string(), loader);
        self
    }
}

impl DocumentLoader for DirectoryLoader {
    fn load(&self, path: &str) -> Result<Vec<Document>> {
        let mut tasks = VecDeque::new();

        let op = self.op.clone();
        let ds = op.list(path)?;
        for de in ds {
            let de = de?;
            let path = de.path().to_string();
            let meta = op.metadata(&de, Metakey::Mode)?;
            match meta.mode() {
                EntryMode::FILE => {
                    for loader in &self.loaders {
                        if glob_match(loader.0, &path) {
                            tasks.push_back((path, loader.1.clone()));
                            break;
                        }
                    }
                }
                EntryMode::DIR => continue,
                EntryMode::Unknown => continue,
            }
        }

        let pool = ThreadPoolBuilder::new().num_threads(4).build()?;

        let results: Vec<_> = pool.install(|| {
            tasks
                .iter()
                .map(|(path, loader)| loader.load(path))
                .collect() // collect results
        });

        let mut documents = vec![];
        for result in results {
            let result = result?;
            documents.extend(result);
        }

        Ok(documents)
    }
}
