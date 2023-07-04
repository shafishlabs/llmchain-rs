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

/// Cloud Object Storage as Disk
///
/// AWS S3
/// Azure Blob
/// Google Cloud Storage
/// Cloudflare R2
/// Wasabi
/// MinIO
/// Alibaba Cloud OSS
/// Tencent Cloud COS
/// Huawei Cloud OBS
use std::sync::Arc;

use anyhow::Result;
use opendal::Operator;

use crate::Disk;

pub struct RemoteDisk {}

impl RemoteDisk {
    pub fn create() -> Result<Arc<Self>> {
        Ok(Arc::new(RemoteDisk {}))
    }
}

impl Disk for RemoteDisk {
    fn get_operator(&self) -> Result<Operator> {
        todo!()
    }
}
