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

use std::fs::File;
use std::io::Write;

use anyhow::Result;
use goldenfile::Mint;

pub struct MintFile {
    file: File,
}

impl MintFile {
    pub fn create(test_dir: &str, golden_file_name: &str) -> Self {
        let curdir = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let testdata_dir = format!("{}/{}", curdir, test_dir);

        let mut mint = Mint::new(testdata_dir);
        let file = mint.new_goldenfile(golden_file_name).unwrap();
        MintFile { file }
    }

    pub fn write(&mut self, text: &str) -> Result<()> {
        writeln!(self.file, "{}", text)?;
        self.file.flush()?;
        Ok(())
    }
}
