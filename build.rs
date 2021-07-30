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

#[cfg(feature = "regenerate")]
mod regenerate {
    use std::fs::read_dir;
    use std::path::Path;

    use anyhow::{anyhow, Result};
    use schemafy_lib::Generator;
    use tempfile::NamedTempFile;

    pub fn regenerate() -> Result<()> {
        for entry in read_dir("src")? {
            let entry = entry?;
            let schema_path = entry.path().join("ignition.json");
            if schema_path.exists() {
                // ignore OUT_DIR and illicitly write back to the source tree
                // https://doc.rust-lang.org/cargo/reference/build-script-examples.html#code-generation
                regenerate_one(&schema_path, &entry.path().join("schema.rs"))?;
            }
        }
        Ok(())
    }

    fn regenerate_one(schema_path: &Path, rust_path: &Path) -> Result<()> {
        let output_file = NamedTempFile::new_in(rust_path.parent().unwrap())?;
        Generator::builder()
            .with_root_name_str("Config")
            .with_input_file(&schema_path)
            .build()
            .generate_to_file(
                &output_file
                    .path()
                    .to_str()
                    .ok_or_else(|| anyhow!("couldn't convert output path"))?,
            )?;
        output_file.persist(rust_path)?;
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    #[cfg(feature = "regenerate")]
    regenerate::regenerate()?;
    Ok(())
}
