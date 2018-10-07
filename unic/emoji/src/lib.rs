// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![forbid(bad_style, future_incompatible, missing_debug_implementations, missing_docs,
          unconditional_recursion, unsafe_code, unused)]

//! # UNIC — Unicode Emoji
//!
//! A component of [`unic`: Unicode and Internationalization Crates for Rust](/unic/).
//!
//! This UNIC component implements character properties and algorithms from [Unicode® Technical
//! Standard #51 - Unicode Emoji](http://unicode.org/reports/tr51/).

pub extern crate unic_emoji_char as char;

mod pkg_info;
pub use self::pkg_info::{PKG_DESCRIPTION, PKG_NAME, PKG_VERSION};

pub use self::char::EMOJI_VERSION;
