# @mochi/cli
> An easy-to-use CLI for compiling and bundling [Mochi](https://github.com/Mochi-Team/mochi) modules.

Powered by <a href="https://github.com/evanw/esbuild"><img src="https://esbuild.github.io/favicon.svg" alt="esbuild" style="max-width: 18px;"/></a>

## Installation

For pnpm:
```
pnpm add -g @mochi/cli
```

For npm:
```
npm install -g @mochi/cli
```

## Usage
```
A CLI tool used to build and bundle Mochi modules.

Options:
  -V, --version                           output the version number
  -h, --help                              display help for command

Commands:
  check [source]                          checks for errors in repository
  bundle [options] [source] [output]      bundles modules into a repository
  serve [options] [source] [destination]  bundle and start local server for testing modules
  help [command]                          display help for command
```

## License
Licenced under [MIT](LICENSE).