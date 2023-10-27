# Cargo.toml + Mochi Config
#
# This file describes Rust's workspace, in addition to defining repository metadata for Mochi.
#
# [workspace.metadata.mochi]
# name: String <Required> - Display name of the repo
# author: String <Required> - Name of the author.
# description: String <Optional> - Description of this repo
# icon: URL-String <Optional> - This must be a valid url link or a relative path to /res/ directory.

[workspace.metadata.mochi]
name = {{ repository.name }}
author = {{ repository.author }}

[workspace]
members = ["modules/*"]
resolver = "2"

[workspace.dependencies]
mochi = { package = "mochi-rs", version = "0.0.2" }
percent-encoding = { version = "2.0.0", default-features = false }

[profile.release]
opt-level = "s"
lto = true
panic = "abort"