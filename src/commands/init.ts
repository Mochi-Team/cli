import consola from 'consola';
import ejs from 'ejs';
import { existsSync } from 'fs';
import { readFile, writeFile, mkdir, readdir, stat, cp } from 'fs/promises';
import path from 'path';
import { retrieveMochiJSVersion, toKebabCase } from '../utils';

export async function handleInitRepo(dir: string, name: string, author: string) {
  if (name.trim().length == 0) {
    throw new Error('Repo name cannot be empty.');
  }

  if (author.trim().length == 0) {
    throw new Error('Author name cannot be empty.');
  }

  const repo = {
    name: name,
    author: author,
  };

  const templateRepoPath = path.resolve(__dirname, '../templates/repo');
  const allFiles = await readdir(templateRepoPath, { recursive: true });

  consola.start('Initializing new repo..');

  dir = path.resolve(path.join(process.cwd(), toKebabCase(name)), dir);
  await mkdir(path.join(dir, 'src'), { recursive: true });

  for (const item of allFiles) {
    const templateItemPath = path.join(templateRepoPath, item);
    const stats = await stat(templateItemPath);

    if (stats.isDirectory()) {
      await mkdir(path.join(dir, item), { recursive: true });
    } else {
      const destinationPath = path.join(dir, item);
      if (item.includes('gitignore')) {
        await cp(templateItemPath, path.join(dir, `.${item}`));
      } else if (item.includes('.ejs')) {
        const template = await readFile(templateItemPath, {
          encoding: 'utf-8',
        });
        const output = await ejs.render(template, repo, { async: true });
        await writeFile(destinationPath.replace('.ejs', ''), output);
      } else {
        await cp(templateItemPath, path.join(dir, item), { recursive: true });
      }
    }
  }

  consola.success(`Successfully created repo in ${dir}`);
}

export async function handleInitModule(dir: string, name: string) {
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
