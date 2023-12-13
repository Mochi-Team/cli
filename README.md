# @mochiapp/cli
> An easy-to-use CLI for compiling and bundling [Mochi](https://github.com/Mochi-Team/mochi) modules.

![npm (scoped)](https://img.shields.io/npm/v/%40mochiapp/cli)
![lint](https://github.com/Mochi-Team/cli/actions/workflows/lint.yml/badge.svg)
![ci](https://github.com/Mochi-Team/cli/actions/workflows/ci.yml/badge.svg)
![cd](https://github.com/Mochi-Team/cli/actions/workflows/cd.yml/badge.svg)

Powered by <a href="https://github.com/evanw/esbuild"><img style="height: 1rem; vertical-align: text-bottom;" src="https://esbuild.github.io/favicon.svg" alt="esbuild logo"/></a>

## Requirements
You must have [Node](https://nodejs.org) installed on your computer and the minimum version supported is `v16.x`.

## Installation

For pnpm:
```bash
# Local cli
pnpm add -D @mochiapp/cli

# Or Global 
pnpm add -g @mochiapp/cli
```

For npm:
```bash
# Local cli
npm install --save-dev @mochiapp/cli

# Or Global 
npm install -g @mochiapp/cli
```

For yarn:
```bash
# Local cli
yarn add -D @mochiapp/cli

# Or Global 
yarn global add @mochiapp/cli
```

## Usage
```
A cli tool used to build and bundle Mochi modules.

Options:
  -V, --version     output the version number
  -h, --help        display help for command

Commands:
  init [options]    initalize a new module from template
  check [options]   checks for errors in repository
  bundle [options]  bundles modules into a repository
  serve [options]   bundle and start local server for testing modules
  help [command]    display help for command
```

## Contribute

You must use [pnpm](https://pnpm.io/) in order to make any changes. Once you've made changes, submit a pull request. Any contributions are welcome and appreciated :D

## License
Licenced under [MIT](LICENSE).