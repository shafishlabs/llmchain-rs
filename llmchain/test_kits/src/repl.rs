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

use std::future::Future;
use std::pin::Pin;

use anyhow::Result;
use rustyline::config::Builder;
use rustyline::error::ReadlineError;
use rustyline::CompletionType;
use rustyline::DefaultEditor;

pub type AsyncCallback =
    dyn Fn(String) -> Pin<Box<dyn Future<Output = Result<String>> + Send>> + Send + Sync;

pub async fn handle_repl(
    hint: &str,
    callback: Box<AsyncCallback>,
) -> Result<(), Box<dyn std::error::Error>> {
    let config = Builder::new()
        .completion_prompt_limit(5)
        .completion_type(CompletionType::Circular)
        .build();

    let mut rl = DefaultEditor::with_config(config)?;

    loop {
        match rl.readline(hint) {
            Ok(line) => {
                println!("{}", (callback)(line).await?);
            }
            Err(e) => match e {
                ReadlineError::Io(err) => {
                    eprintln!("io err: {err}");
                    return Err(Box::new(err));
                }
                ReadlineError::Interrupted => {
                    println!("^C");
                }
                ReadlineError::Eof => {
                    break;
                }
                _ => {}
            },
        }
    }
    println!("Bye~");

    Ok(())
}
