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
    use std::io::{Read, Write};
    use std::path::Path;
    use std::process::{Command, Stdio};

    use anyhow::{anyhow, bail, Context, Result};
    use proc_macro2::{Ident, Span};
    use quote::quote;
    use schemafy_lib::Generator;
    use syn::visit_mut::{self, VisitMut};
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
        // generate the Rust code
        let mut generated_file = NamedTempFile::new()?;
        Generator::builder()
            .with_root_name_str("Config")
            .with_input_file(&schema_path)
            .build()
            .generate_to_file(
                &generated_file
                    .path()
                    .to_str()
                    .ok_or_else(|| anyhow!("couldn't convert output path"))?,
            )?;

        // parse it back
        let mut buf = String::new();
        generated_file.read_to_string(&mut buf)?;
        let mut tree = syn::parse_file(&buf)?;

        // modify it
        Rewriter.visit_file_mut(&mut tree);

        // run it through rustfmt and write it out
        let output = format!("// Generated code; do not modify\n\n{}", quote!(#tree));
        let (output_file, output_path) =
            NamedTempFile::new_in(rust_path.parent().unwrap())?.into_parts();
        let mut formatter = Command::new("rustfmt")
            .args(&["--edition", "2018"])
            .stdin(Stdio::piped())
            .stdout(output_file)
            .spawn()
            .context("couldn't run rustfmt")?;
        formatter
            .stdin
            .as_mut()
            .expect("stdin")
            .write_all(output.as_bytes())?;
        let result = formatter.wait()?;
        if !result.success() {
            bail!("rustfmt failed");
        }
        output_path.persist(rust_path)?;

        Ok(())
    }

    struct Rewriter;

    impl VisitMut for Rewriter {
        fn visit_field_mut(&mut self, node: &mut syn::Field) {
            if let Some(ident) = node.ident.clone() {
                // mangle field identifier
                let name = ident.to_string();
                node.ident = Some(match name.as_str() {
                    // Inflector converts tpm2 to tpm_2
                    "tpm_2" => Ident::new("tpm2", Span::call_site()),
                    // Inflector converts MiB to mi_b
                    s if s.ends_with("_mi_b") => {
                        let new_name = name.replace("_mi_b", "_mib");
                        Ident::new(&new_name, Span::call_site())
                    }
                    _ => ident,
                })
            }
            visit_mut::visit_field_mut(self, node);
        }

        fn visit_path_segment_mut(&mut self, node: &mut syn::PathSegment) {
            // mangle struct identifier in field type

            // coalesce node user/group structs
            // https://github.com/Marwes/schemafy/issues/50
            let name = node.ident.to_string();
            let new_name = match name.as_str() {
                "DirectoryGroup" => "NodeGroup",
                "DirectoryUser" => "NodeUser",
                "FileGroup" => "NodeGroup",
                "FileUser" => "NodeUser",
                "LinkGroup" => "NodeGroup",
                "LinkUser" => "NodeUser",
                _ => &name,
            }
            .to_string();

            if new_name != name {
                node.ident = Ident::new(&new_name, Span::call_site())
            }
            visit_mut::visit_path_segment_mut(self, node);
        }

        fn visit_file_mut(&mut self, node: &mut syn::File) {
            // drop definitions for now-unused user/group structs
            // https://github.com/Marwes/schemafy/issues/50
            visit_mut::visit_file_mut(self, node);
            node.items = node
                .items
                .drain(..)
                .filter(|item| match item {
                    syn::Item::Struct(s) => !matches!(
                        s.ident.to_string().as_str(),
                        "DirectoryGroup"
                            | "DirectoryUser"
                            | "FileGroup"
                            | "FileUser"
                            | "LinkGroup"
                            | "LinkUser"
                    ),
                    _ => true,
                })
                .collect();
        }
    }
}

fn main() -> anyhow::Result<()> {
    #[cfg(feature = "regenerate")]
    regenerate::regenerate()?;
    Ok(())
}
