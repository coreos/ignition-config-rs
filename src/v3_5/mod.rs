// Copyright 2021 Red Hat, Inc.
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

// schemafy doesn't derive Eq on structs, and we don't do it either because
// any floating point value added to a struct in a future schema version
// would cause cascading Eq removals.  PartialEq is good enough for now.
// https://github.com/Marwes/schemafy/issues/61
#![allow(clippy::derive_partial_eq_without_eq)]

use semver::Version;
use serde::{Deserialize, Serialize};

pub(crate) const VERSION: Version = Version::new(3, 5, 0);

include!("schema.rs");

// clippy doesn't know that the derive would need to be in generated code
#[allow(clippy::derivable_impls)]
impl Default for Config {
    fn default() -> Self {
        Self {
            ignition: Default::default(),
            kernel_arguments: None,
            passwd: None,
            storage: None,
            systemd: None,
        }
    }
}

impl Default for Ignition {
    fn default() -> Self {
        Self {
            config: None,
            proxy: None,
            security: None,
            timeouts: None,
            version: VERSION.to_string(),
        }
    }
}
