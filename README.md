# @mochi/cli
> An easy-to-use CLI for compiling and bundling [Mochi](https://github.com/Mochi-Team/mochi) modules.

Powered by <a href="https://github.com/evanw/esbuild"><img src="https://esbuild.github.io/favicon.svg" alt="esbuild" style="max-width: 18px;"/></a>

![lint](https://github.com/Mochi-Team/cli/actions/workflows/lint/badge.svg)
![ci](https://github.com/Mochi-Team/cli/actions/workflows/ci/badge.svg)
![cd](https://github.com/Mochi-Team/cli/actions/workflows/cd/badge.svg)

## Installation

For pnpm:
```bash
pnpm add -g @mochi/cli
```

For npm:
```bash
npm install -g @mochi/cli
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