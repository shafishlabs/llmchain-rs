// Copyright 2024 Shafish Labs.
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


#[derive(Clone)]
pub enum ModelType {
    Mixtral,
    OpenChat35,
}

pub struct Repo {
    pub repo_name: String,
    pub file_name: String,
    pub size:String
}

impl ModelType {

    /// Returns the model name and the file name of the model
    pub fn repo(&self) -> Repo {
        match self {
            ModelType::Mixtral => {
                Repo {
                    repo_name: "TheBloke/Mixtral-8x7B-v0.1-GGUF".to_string(),
                    file_name: "mixtral-8x7b-v0.1.Q4_K_M.gguf".to_string(),
                    size: "26.4 GB".to_string()
                }
            }
            ModelType::OpenChat35 => {
                Repo {
                    repo_name: "TheBloke/openchat_3.5-GGUF".to_string(),
                    file_name: "openchat_3.5.Q4_K_M.gguf".to_string(),
                    size: "4.37 GB".to_string()
                }
            }
        }
    }
}


