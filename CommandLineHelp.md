# Command-Line Help for `mochi-cli`

This document contains the help content for the `mochi-cli` command-line program.

**Command Overview:**

* [`mochi-cli`↴](#mochi-cli)
* [`mochi-cli init`↴](#mochi-cli-init)
* [`mochi-cli init module`↴](#mochi-cli-init-module)
* [`mochi-cli init repository`↴](#mochi-cli-init-repository)
* [`mochi-cli build`↴](#mochi-cli-build)
* [`mochi-cli serve`↴](#mochi-cli-serve)

## `mochi-cli`

A CLI tool for managing mochi-based modules.

**Usage:** `mochi-cli <COMMAND>`

###### **Subcommands:**

* `init` — Initializes a repository or module
* `build` — Builds repository from modules
* `serve` — Builds repository and starts a local server



## `mochi-cli init`

Initializes a repository or module

**Usage:** `mochi-cli init <COMMAND>`

###### **Subcommands:**

* `module` — Creates a module from a template
* `repository` — Creates a repository from a template



## `mochi-cli init module`

Creates a module from a template

**Usage:** `mochi-cli init module --name <NAME>`

###### **Options:**

* `-n`, `--name <NAME>` — Display name for the module



## `mochi-cli init repository`

Creates a repository from a template

**Usage:** `mochi-cli init repository [OPTIONS] --name <NAME> --author <AUTHOR>`

###### **Options:**

* `--name <NAME>` — Display name for the repository
* `--author <AUTHOR>` — Author of the repository
* `--output <OUTPUT>` — Output path to the repository. By default it uses the cwd + `/repository-name/`



## `mochi-cli build`

Builds repository from modules

**Usage:** `mochi-cli build [OPTIONS]`

###### **Options:**

* `--path <PATH>` — Path of workspace
* `--output <OUTPUT>` — Output path for generated repository. For more info use `--help`
* `-s`, `--site` — Include generated static site for repository

  Default value: `false`



## `mochi-cli serve`

Builds repository and starts a local server

**Usage:** `mochi-cli serve [OPTIONS]`

###### **Options:**

* `--port <PORT>` — The port to broadcast the repository (default is 10443)
* `--output <OUTPUT>` — The repository output (default is "dist")



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>
