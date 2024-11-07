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

use semver::Version;
use serde::{Deserialize, Serialize};
use serde_ignored::Path;

pub mod v3_0;
pub mod v3_1;
pub mod v3_2;
pub mod v3_3;
pub mod v3_4;
pub mod v3_5;

type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("couldn't parse config version: {0}")]
    InvalidVersion(#[from] semver::Error),
    #[error("unsupported config version: {0}")]
    UnknownVersion(Version),
}

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Warning {
    #[error("unused key: {0}")]
    UnusedKey(String),
}

// can't implement Deserialize since that consumes the input stream and we
// need to parse twice
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum Config {
    V3_0(v3_0::Config),
    V3_1(v3_1::Config),
    V3_2(v3_2::Config),
    V3_3(v3_3::Config),
    V3_4(v3_4::Config),
    V3_5(v3_5::Config),
}

impl Config {
    pub fn parse_str(s: &str) -> Result<(Self, Vec<Warning>)> {
        Self::parse_slice(s.as_bytes())
    }

    pub fn parse_slice(v: &[u8]) -> Result<(Self, Vec<Warning>)> {
        let minimal: MinimalConfig = serde_json::from_slice(v)?;
        let version = Version::parse(&minimal.ignition.version)?;
        let mut warnings = Vec::new();
        // can't use match because of some implementation details of Version
        let parsed = if version == v3_0::VERSION {
            Self::V3_0(parse_warn(v, &mut warnings)?)
        } else if version == v3_1::VERSION {
            Self::V3_1(parse_warn(v, &mut warnings)?)
        } else if version == v3_2::VERSION {
            Self::V3_2(parse_warn(v, &mut warnings)?)
        } else if version == v3_3::VERSION {
            Self::V3_3(parse_warn(v, &mut warnings)?)
        } else if version == v3_4::VERSION {
            Self::V3_4(parse_warn(v, &mut warnings)?)
        } else if version == v3_5::VERSION {
            Self::V3_5(parse_warn(v, &mut warnings)?)
        } else {
            return Err(Error::UnknownVersion(version));
        };
        Ok((parsed, warnings))
    }
}

#[derive(Debug, Deserialize)]
struct MinimalConfig {
    ignition: MinimalIgnition,
}

#[derive(Debug, Deserialize)]
struct MinimalIgnition {
    version: String,
}

/// Deserialize and populate warnings.
fn parse_warn<'de, T: Deserialize<'de>>(v: &'de [u8], warnings: &mut Vec<Warning>) -> Result<T> {
    Ok(serde_ignored::deserialize(
        &mut serde_json::Deserializer::from_slice(v),
        |path| warnings.push(Warning::UnusedKey(path_string(&path))),
    )?)
}

/// Convert Path to String using vcontext-style formatting.
/// In particular, don't add a ? to Option<> wrappers, as Path.to_string()
/// does.
fn path_string(path: &Path) -> String {
    use Path::*;
    match path {
        Root => "$".into(),
        Seq { parent, index } => format!("{}.{}", path_string(parent), index),
        Map { parent, key } => format!("{}.{}", path_string(parent), key),
        Some { parent } => path_string(parent),
        NewtypeStruct { parent } => path_string(parent),
        NewtypeVariant { parent } => path_string(parent),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        assert!(matches!(
            Config::parse_str("z").unwrap_err(),
            Error::Serialization(_)
        ));
        assert!(matches!(
            Config::parse_str("{}").unwrap_err(),
            Error::Serialization(_)
        ));
        assert!(matches!(
            Config::parse_str(r#"{"ignition": {"version": "z"}}"#).unwrap_err(),
            Error::InvalidVersion(_)
        ));
        assert!(matches!(
            Config::parse_str(r#"{"ignition": {"version": "2.0.0"}}"#).unwrap_err(),
            Error::UnknownVersion(_)
        ));
        assert!(matches!(
            Config::parse_str(r#"{"ignition": {"version": "3.0.0-experimental"}}"#).unwrap_err(),
            Error::UnknownVersion(_)
        ));

        let mut expected = v3_0::Config::default();
        expected
            .storage
            .get_or_insert_with(Default::default)
            .files
            .get_or_insert_with(Default::default)
            .push(v3_0::File::new("/z".into()));
        let (config, warnings) = Config::parse_str(
            r#"{"ignition": {"version": "3.0.0"}, "storage": {"files": [{"path": "/z"}]}}"#,
        )
        .unwrap();
        assert_eq!(config, Config::V3_0(expected));
        assert!(warnings.is_empty());

        let mut expected = v3_1::Config::default();
        expected
            .storage
            .get_or_insert_with(Default::default)
            .files
            .get_or_insert_with(Default::default)
            .push(v3_1::File::new("/z".into()));
        let (config, warnings) = Config::parse_str(
            r#"{"ignition": {"version": "3.1.0"}, "storage": {"files": [{"path": "/z"}]}}"#,
        )
        .unwrap();
        assert_eq!(config, Config::V3_1(expected));
        assert!(warnings.is_empty());

        let mut expected = v3_2::Config::default();
        expected
            .storage
            .get_or_insert_with(Default::default)
            .files
            .get_or_insert_with(Default::default)
            .push(v3_2::File::new("/z".into()));
        let (config, warnings) = Config::parse_str(
            r#"{"ignition": {"version": "3.2.0"}, "storage": {"files": [{"path": "/z"}]}}"#,
        )
        .unwrap();
        assert_eq!(config, Config::V3_2(expected));
        assert!(warnings.is_empty());

        let mut expected = v3_3::Config::default();
        expected
            .storage
            .get_or_insert_with(Default::default)
            .files
            .get_or_insert_with(Default::default)
            .push(v3_3::File::new("/z".into()));
        let (config, warnings) = Config::parse_str(
            r#"{"ignition": {"version": "3.3.0"}, "storage": {"files": [{"path": "/z"}]}}"#,
        )
        .unwrap();
        assert_eq!(config, Config::V3_3(expected));
        assert!(warnings.is_empty());

        let mut expected = v3_4::Config::default();
        expected
            .storage
            .get_or_insert_with(Default::default)
            .files
            .get_or_insert_with(Default::default)
            .push(v3_4::File::new("/z".into()));
        let (config, warnings) = Config::parse_str(
            r#"{"ignition": {"version": "3.4.0"}, "storage": {"files": [{"path": "/z"}]}}"#,
        )
        .unwrap();
        assert_eq!(config, Config::V3_4(expected));
        assert!(warnings.is_empty());
    }

    #[test]
    fn round_trip() {
        let input = r#"{"ignition":{"version":"3.0.0"}}"#;
        let (config, warnings) = Config::parse_str(input).unwrap();
        assert_eq!(serde_json::to_string(&config).unwrap(), input);
        assert!(warnings.is_empty());
    }

    #[test]
    fn warnings() {
        let (_, warnings) = Config::parse_str(
            r#"{"ignition": {"version": "3.0.0"}, "a": {"y": "z"}, "b": 7, "c": null, "systemd": {"units": [{"name": "v", "d": "e"}]}}"#,
        )
        .unwrap();
        assert_eq!(
            warnings
                .iter()
                .map(|w| w.to_string())
                .collect::<Vec<String>>(),
            vec![
                "unused key: $.a",
                "unused key: $.b",
                "unused key: $.c",
                "unused key: $.systemd.units.0.d",
            ]
        );
    }
}
