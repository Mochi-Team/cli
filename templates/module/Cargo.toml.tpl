# Cargo + Mochi Manifest
#
# Cargo.toml is used as a configuration file for rust. In this case, we are also using it as a
# a way to retrieve mochi sources. Below are the required and optinal fields for a working Mochi module.
#
# [package.metadata.mochi]
# - name: String [Required] The display name of the module.
# - description: String [Optional] Description of the module.
# - icon: URL-String [Optional] A url based icon. The url can be an absolute link only if you're using a remote link. 
# It can also a relative link but it will be relative to `./res` folder.
# 
# [package] (this is usually requierd by rust anyways)
# - version: String - This must follow Semantic versioning 2.0.0, (https://semver.org/) 
# Any changes you decide to make to the module, you will need to bump the package version so users can be notified of a new update.
#
# Example:
#
# [package.metadata.mochi]
# name = "module"
# description = "This is a basic module"
# icon = "basic-module.png" - This will be converted to `./res/basic-module.png` if using relative.

[package.metadata.mochi]
name = "{{ module.display_name }}"

[package]
name = "{{ module.identifier_name }}"
version = "0.0.1"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
mochi = { workspace = true, features = ["extractors"] }
percent-encoding = { workspace = true }