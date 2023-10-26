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

* `module` — 
* `repository` — 



## `mochi-cli init module`

**Usage:** `mochi-cli init module`



## `mochi-cli init repository`

**Usage:** `mochi-cli init repository`



## `mochi-cli build`

Builds repository from modules

**Usage:** `mochi-cli build [OPTIONS]`

###### **Options:**

* `--path <PATH>` — Path of workspace. (Defaults to current working directory)
* `--output <OUTPUT>` — Output path for generated repository. For more info use `--help`
* `-s`, `--site` — Include generated static site for repository

  Default value: `false`



## `mochi-cli serve`

Builds repository and starts a local server

**Usage:** `mochi-cli serve [OPTIONS]`

###### **Options:**

* `--port <PORT>`
* `--output <OUTPUT>`



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>
