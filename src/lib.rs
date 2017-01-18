// src/lib.rs -- main module file for the Tectonic library.
// Copyright 2016-2017 the Tectonic Project
// Licensed under the MIT License.

#![recursion_limit = "1024"] // "error_chain can recurse deeply"

extern crate app_dirs;
extern crate crypto;
#[macro_use]
extern crate error_chain;
extern crate flate2;
extern crate hyper;
extern crate libc;
extern crate md5; // TODO: eliminate in favor of just crypto
extern crate mkstemp;
extern crate zip;

pub mod engines;
pub mod errors;
pub mod io;

pub use engines::tex::{TexEngine, TexResult};
pub use engines::xdvipdfmx::XdvipdfmxEngine;
pub use errors::{Error, ErrorKind, Result};
