
////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2023. Rob Bailey                                              /
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

mod rule;
mod logging;
mod library;

use std::fmt;
use std::str::FromStr;
use serde::{de, Deserialize, Deserializer, Serialize};
use serde::de::{Error, MapAccess, SeqAccess, Visitor};
use library::Library;
use logging::Logging;
use rule::Rule;
use crate::VersionKind;


#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize)]
pub struct Argument {
    pub rules: Vec<Rule>,
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct ArrayOrStringHelper(pub Vec<String>);

/// deserialize either an array of strings or a single string into always a vector of strings
impl<'de> Deserialize<'de> for ArrayOrStringHelper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        struct ArrayOrStringVisitor;

        impl<'de> Visitor<'de> for ArrayOrStringVisitor {
            type Value = ArrayOrStringHelper;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("string or array of strings")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
                where E: de::Error
            {
                Ok(ArrayOrStringHelper(vec![s.to_owned()]))
            }

            fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
                where S: SeqAccess<'de>
            {
                let mut vec = Vec::new();
                while let Some(elem) = seq.next_element::<String>()? {
                    vec.push(elem);
                }
                Ok(ArrayOrStringHelper(vec))
            }
        }

        deserializer.deserialize_any(ArrayOrStringVisitor)
    }
}

impl FromStr for Argument {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Argument {
            rules: vec![],
            values: vec![s.to_owned()],
        })
    }
}

impl<'de> Deserialize<'de> for Argument {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        struct ArgumentVisitor;

        impl<'de> Visitor<'de> for ArgumentVisitor {
            type Value = Argument;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("string or object with rules and value fields")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
                where E: de::Error
            {
                Ok(Argument {
                    rules: vec![],
                    values: vec![s.to_owned()],
                })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
                where M: MapAccess<'de>
            {
                let mut rules = None;
                let mut value = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "rules" => {
                            if rules.is_some() {
                                return Err(de::Error::duplicate_field("rules"));
                            }
                            rules = Some(map.next_value()?);
                        }
                        "value" => {
                            if value.is_some() {
                                return Err(de::Error::duplicate_field("value"));
                            }
                            value = Some(map.next_value::<ArrayOrStringHelper>()?.0);
                        }
                        _ => {
                            return Err(Error::unknown_field(&key, &["rules", "value"]));
                        }
                    }
                }

                let rules = rules.ok_or_else(|| de::Error::missing_field("rules"))?;
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;

                Ok(Argument {
                    rules,
                    values: value,
                })
            }
        }

        deserializer.deserialize_any(ArgumentVisitor)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Arguments {
    pub game: Vec<Argument>,
    pub jvm: Vec<Argument>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AssetIndex {
    pub id: String,
    pub sha1: String,
    pub size: u64,
    pub total_size: u64,
    pub url: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Download {
    pub sha1: String,
    pub size: u64,
    pub url: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Downloads {
    pub client: Download,
    #[serde(default)]
    pub client_mappings: Option<Download>,
    #[serde(default)]
    pub server: Option<Download>,
    #[serde(default)]
    pub server_mappings: Option<Download>,
    #[serde(default)]
    pub windows_server: Option<Download>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JavaVersion {
    pub component: String,
    pub major_version: u8,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Version {
    #[serde(default)]
    pub arguments: Option<Arguments>,
    #[serde(default)]
    pub minecraft_arguments: Option<String>,
    pub asset_index: AssetIndex,
    pub assets: String,
    #[serde(default)]
    pub compliance_level: Option<u8>,
    pub downloads: Downloads,
    pub id: String,
    #[serde(default)]
    pub java_version: Option<JavaVersion>,
    pub libraries: Vec<Library>,
    #[serde(default)]
    pub logging: Option<Logging>,
    pub main_class: String,
    pub minimum_launcher_version: u8,
    pub release_time: String,
    pub time: String,
    #[serde(rename = "type")]
    pub kind: VersionKind,
}
