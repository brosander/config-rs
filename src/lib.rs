//! Config organizes hierarchical or layered configurations for Rust applications.
//!
//! Config lets you set a set of default parameters and then extend them via merging in
//! configuration from a variety of sources:
//!  - Environment variables
//!  - Another Config instance
//!  - Remote configuration: etcd, Consul
//!  - Files: JSON, YAML, TOML
//!  - Manual, programmatic override (via a `.set` method on the Config instance)
//!
//! Additionally, Config supports:
//!  - Live watching and re-reading of configuration files
//!  - Deep access into the merged configuration via a path syntax
//!  - Deserialization via `serde` of the configuration or any subset defined via a path
//!
//! See the [examples](https://github.com/mehcode/config-rs/tree/master/examples) for
//! general usage information.
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unknown_lints)]
// #![warn(missing_docs)]

#[macro_use]
extern crate serde;

extern crate nom;

extern crate thread_local;

#[macro_use]
extern crate lazy_static;

#[cfg(feature = "toml")]
extern crate toml;

#[cfg(feature = "json")]
extern crate serde_json;

#[cfg(feature = "yaml")]
extern crate yaml_rust;

#[cfg(any(feature = "remote-etcd", feature = "remote-etcd-tls"))]
extern crate futures;

#[cfg(any(feature = "remote-etcd", feature = "remote-etcd-tls"))]
extern crate etcd;

#[cfg(any(feature = "remote-etcd", feature = "remote-etcd-tls"))]
extern crate tokio_core;

#[cfg(any(feature = "remote-etcd", feature = "remote-etcd-tls"))]
extern crate hyper;

#[cfg(feature = "remote-etcd-tls")]
extern crate hyper_tls;

#[cfg(feature = "remote-consul")]
extern crate consul;

mod error;
mod value;
mod de;
mod path;
mod source;
mod config;
mod file;
mod env;
pub mod remote;

pub use config::Config;
pub use error::ConfigError;
pub use value::{Array, Table, Value};
pub use source::Source;
pub use file::{File, FileFormat};
pub use env::Environment;
pub use remote::Remote;
