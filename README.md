[![Crates.io Version](https://img.shields.io/crates/v/vcheat?style=for-the-badge)](https://crates.io/crates/vcheat)
[![Static Badge](https://img.shields.io/badge/Github-vcheat-green?style=for-the-badge)](https://github.com/sylvavv/vcheat/)

Hacking Library

```rust
// tests/external.rs

 let pid = vcheat::external::get_pid("explorer.exe").unwrap();

 let mi = vcheat::external::get_mod_info(pid, "explorer.exe").unwrap();
 println!("{:#?}", mi);

 let mis = vcheat::external::get_all_mod_info(pid).unwrap();

 for mi in mis {
     println!("{:#?}", mi);
 }

```
