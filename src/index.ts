#! /usr/bin/env node

import { Command } from '@commander-js/extra-typings';
import { version as cliVersion } from '../package.json';
import handleBuild from './commands/check';
import handleBundle from './commands/bundle';
import handleServe from './commands/serve';
import consola from 'consola';

const program = new Command();

program
    .version(cliVersion)
    .description("A CLI tool used to build and bundle Mochi modules.");

program
    .command('check')
    .argument('[source]')
    .description('checks for errors in repository')
    .action(src => handleBuild(src).catch(e => consola.error(e)));

program
    .command('bundle')
    .argument('[source]')
    .argument('[output]')
    .option('-s, --site', 'generate static site')
    .description('bundles modules into a repository')
    .action((src, dest, options) => handleBundle(src, dest, options.site).catch(e => consola.error(e)));

program
    .command('serve')
    .argument('[source]')
    .argument('[destination]')
    .option('-s, --site', 'generate static site')
    .description('bundle and start local server for testing modules')
    .action((src, dest, options) => handleServe(src, dest, options.site).catch(e => consola.error(e)));

program.parse(process.argv);