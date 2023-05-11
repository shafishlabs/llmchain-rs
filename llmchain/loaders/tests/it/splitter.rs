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

use llmchain_loaders::document::DocumentSettings;
use llmchain_loaders::splitter::TextSplitter;

struct CustomTextSplitter {
    settings: DocumentSettings,
}

impl CustomTextSplitter {
    fn new(splitter_chunk_size: usize) -> Self {
        CustomTextSplitter {
            settings: DocumentSettings {
                splitter_chunk_size,
            },
        }
    }
}

impl TextSplitter for CustomTextSplitter {
    fn separators(&self) -> Vec<String> {
        vec![String::from("\n"), String::from("--")]
    }

    fn settings(&self) -> DocumentSettings {
        self.settings.clone()
    }
}

#[test]
fn test_split() {
    let txt = "Hello\nworld\nThis--is\na test bala bala bala--of text splitting.";

    // splitter_chunk_size=7
    {
        let splitter = CustomTextSplitter::new(7);

        let result = splitter.split_text(txt).unwrap();
        let expected = vec![
            "Hello world",
            "This is a test bala bala bala",
            " of text splitting.",
        ];

        assert_eq!(result, expected);
    }

    // splitter_chunk_size=10
    {
        let splitter = CustomTextSplitter::new(11);

        let result = splitter.split_text(txt).unwrap();
        let expected = vec![
            "Hello world This",
            "is a test bala bala bala",
            " of text splitting.",
        ];

        assert_eq!(result, expected);
    }
}
