import consola from 'consola';
import ejs from 'ejs';
import { existsSync } from 'fs';
import { readFile, writeFile, mkdir } from 'fs/promises';
import path from 'path';
import { retrieveMochiJSVersion } from '../utils';

export default async function handleInit(dir: string, name: string) {
  consola.start('Initializing new module...');

  if (name.trim().length == 0) {
    throw new Error('Module name cannot be empty.');
  }

  dir = path.resolve(process.cwd(), dir);

  await retrieveMochiJSVersion(dir);

  const srcDir = path.join(dir, 'src');
  const casePathName = toPascalCase(name);

  const module = {
    className: casePathName,
    displayName: name,
  };

  // Check if there's no directory with the same name
  const moduleDir = path.join(srcDir, casePathName.toLowerCase());
  if (existsSync(moduleDir)) throw new Error(`cannot create module with name ${name}: directory exists.`);

  const template = await readFile(path.resolve(__dirname, '../templates/module/index.ejs'), {
    encoding: 'utf-8',
  });

  const output = await ejs.render(template, module, { async: true });

  await mkdir(moduleDir, { recursive: true });
  await writeFile(path.join(moduleDir, 'index.ts'), output);
  await mkdir(path.join(moduleDir, 'res'));

  consola.log('');
  consola.success(`Successfully created module in ${moduleDir}`);
}

const toPascalCase = (str: string): string =>
  str
    .match(/[A-Z]{2,}(?=[A-Z][a-z]+[0-9]*|\b)|[A-Z]?[a-z]+[0-9]*|[A-Z]|[0-9]+/g)
    ?.map((x) => x.charAt(0).toUpperCase() + x.slice(1).toLowerCase())
    .join('') ?? '';
