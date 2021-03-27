# rust-backup
A backup tool in Rust. 

rust-backup detects changes in the specified source directory using a hash value, and if it detects a changes, it zip the directory and save it in the specified destination directory.

rust-backup is configured using a TOML file.

## TODO
* Add other compression methods
* Add log output
