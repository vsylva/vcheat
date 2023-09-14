//! # 中文
//!
//! [![Crates.io](https://img.shields.io/crates/v/vcheat)](https://crates.io/crates/vcheat)
//!
//! "vcheat" 是用 Rust 语言编写的适用于 Windows 平台的进程作弊库
//!
//! ```example
//! // https://github.com/vSylva/vcheat/tree/main/examples
//! cargo run --example
//! ```
//!
//! # English
//!
//! [![Crates.io](https://img.shields.io/crates/v/vcheat)](https://crates.io/crates/vcheat)
//!
//! "vcheat" is a process cheating library designed for the Windows platform and written in Rust programming language
//!
//! ```example
//! // https://github.com/vSylva/vcheat/tree/main/examples
//! cargo run --example
//! ```

mod core;
mod exports;
mod ffi;

pub use exports::*;

type Result<T> = ::core::result::Result<T, String>;
