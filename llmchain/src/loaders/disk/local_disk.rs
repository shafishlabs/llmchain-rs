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

/// Local File System Disk
use std::sync::Arc;

use anyhow::Result;
use opendal::services::Fs;
use opendal::Operator;

use crate::Disk;

pub struct LocalDisk {
    op: Operator,
}

impl LocalDisk {
    pub fn create() -> Result<Arc<Self>> {
        let mut builder = Fs::default();
        builder.root("/");
        let op = Operator::new(builder)?.finish();

        Ok(Arc::new(LocalDisk { op }))
    }
}

impl Disk for LocalDisk {
    fn get_operator(&self) -> Result<Operator> {
        Ok(self.op.clone())
    }
}
