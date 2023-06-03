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

#[derive(Debug)]
pub enum DocumentPath {
    Str(String),
    Range(usize, usize),
}

impl DocumentPath {
    pub fn as_str(&self) -> Result<&str> {
        match self {
            DocumentPath::Str(s) => Ok(s),
            _ => {
                anyhow::bail!("DocumentPath is not a string, {:?}", self)
            }
        }
    }

    pub fn from_string(s: &str) -> Self {
        DocumentPath::Str(s.to_string())
    }

    pub fn as_range(&self) -> Result<(usize, usize)> {
        match self {
            DocumentPath::Range(start, end) => Ok((*start, *end)),
            _ => {
                anyhow::bail!("DocumentPath is not a range, {:?}", self)
            }
        }
    }

    pub fn from_range(start: usize, end: usize) -> Self {
        DocumentPath::Range(start, end)
    }
}
