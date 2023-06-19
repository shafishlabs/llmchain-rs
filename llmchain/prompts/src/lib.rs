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

mod document_retrieval_prompt;
mod github_pr_summary_prompt;
mod prompt;
mod text_to_sql_prompt;

pub use document_retrieval_prompt::DocumentRetrievalPrompt;
pub use github_pr_summary_prompt::GithubPRSummaryPrompt;
pub use prompt::Prompt;
pub use prompt::PromptTemplate;
pub use text_to_sql_prompt::TextToSQLPrompt;
