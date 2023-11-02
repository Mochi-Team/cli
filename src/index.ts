#! /usr/bin/env node

import { Command } from '@commander-js/extra-typings';
import { version, description } from '../package.json';
import handleCheck from './commands/check';
import handleBundle from './commands/bundle';
import handleServe from './commands/serve';
import consola from 'consola';

const program = new Command();

program
  .name('mochi-cli')
  .version(version)
  .description(description);

program
  .command('check')
  .description('checks for errors in repository')
  .option('--dir <DIR>', 'repository path', '.')
  .action(options => handleCheck(options?.dir).catch(writeErrorToConsola));

program
  .command('bundle')
  .description('bundles modules into a repository')
  .option('--dir <DIR>', 'repository path', '.')
  .option('--out <OUT>', 'path to store the bundle', './dist')
  .option('-s, --site', 'generate static site', false)
  .action(options => handleBundle(options.dir, options.out, options.site).catch(writeErrorToConsola));

program
  .command('serve')
  .description('bundle and start local server for testing modules')
  .option('--dir <DIR>', 'repository path', '.')
  .option('--out <OUT>', 'path to store the bundle', './dist')
  .option('--port <PORT>', 'the server port', '10443')
  .option('-s, --site', 'generate static site', false)
  .option('-w, --watch', 'watch repository changes and rebuild', false)
  .action(options => handleServe(options.dir, options.out, options.site, options.watch, options.port).catch(writeErrorToConsola));

program.parse(process.argv);

function writeErrorToConsola(error: any) {
  // consola.log('');
  consola.error(error);
}
