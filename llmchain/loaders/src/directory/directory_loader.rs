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
    loaders: HashMap<String, Arc<dyn DocumentLoader>>,
    max_worker: usize,
}

impl DirectoryLoader {
    pub fn create(op: BlockingOperator) -> Self {
        DirectoryLoader {
            op,
            loaders: HashMap::default(),
            max_worker: 8,
        }
    }

    pub fn with_loader(mut self, glob: &str, loader: Arc<dyn DocumentLoader>) -> Self {
        self.loaders.insert(glob.to_string(), loader);
        self
    }

    pub fn with_max_worker(mut self, max_worker: usize) -> Self {
        self.max_worker = max_worker;
        self
    }

    fn process_directory(
        &self,
        path: &str,
        tasks: &mut VecDeque<(String, Arc<dyn DocumentLoader>)>,
    ) -> Result<()> {
        let op = self.op.clone();
        let ds = op.scan(path)?;
        for de in ds {
            let de = de?;
            let path_buf = de.path();
            let path_str = path_buf.to_string();
            let meta = op.metadata(&de, Metakey::Mode)?;
            match meta.mode() {
                EntryMode::FILE => {
                    for loader in &self.loaders {
                        if glob_match(loader.0, &path_str) {
                            tasks.push_back((path_str, loader.1.clone()));
                            break;
                        }
                    }
                }
                _ => continue,
            }
        }
        Ok(())
    }
}

impl DocumentLoader for DirectoryLoader {
    fn load(&self, path: &str) -> Result<Vec<Document>> {
        let mut tasks: VecDeque<(String, Arc<dyn DocumentLoader>)> = VecDeque::new();
        self.process_directory(path, &mut tasks)?;

        let worker_pool = ThreadPoolBuilder::new()
            .num_threads(self.max_worker)
            .build()?;
        let results: Vec<_> = worker_pool.install(|| {
            tasks
                .iter()
                .map(|(path, loader)| loader.load(path))
                .collect()
        });

        let mut documents = vec![];
        for result in results {
            let result = result?;
            documents.extend(result);
        }

        Ok(documents)
    }
}
