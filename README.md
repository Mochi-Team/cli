# @mochi/cli
> An easy-to-use CLI for compiling and bundling [Mochi](https://github.com/Mochi-Team/mochi) modules.

![npm (scoped)](https://img.shields.io/npm/v/%40mochi/cli)
![lint](https://github.com/Mochi-Team/cli/actions/workflows/lint.yml/badge.svg)
![ci](https://github.com/Mochi-Team/cli/actions/workflows/ci.yml/badge.svg)
![cd](https://github.com/Mochi-Team/cli/actions/workflows/cd.yml/badge.svg)

Powered by <a href="https://github.com/evanw/esbuild"><img style="height: 1rem; vertical-align: text-bottom;" src="https://esbuild.github.io/favicon.svg" alt="esbuild logo"/></a>

## Installation

For pnpm:
```bash
pnpm add -g @mochi/cli
```

For npm:
```bash
npm install -g @mochi/cli
```

For yarn:
```bash
yarn global add @mochi/cli
```

## Usage
```
A cli tool used to build and bundle Mochi modules.

Options:
  -V, --version     output the version number
  -h, --help        display help for command

Commands:
  check [options]   checks for errors in repository
  bundle [options]  bundles modules into a repository
  serve [options]   bundle and start local server for testing modules
  help [command]    display help for command
```

## License
Licenced under [MIT](LICENSE).