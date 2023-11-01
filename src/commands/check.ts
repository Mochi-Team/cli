import * as path from 'path';
import ts from 'typescript';
import { readFile } from 'fs/promises';
import { consola } from 'consola';
import { directoryExists, getModulesDirectories, retrieveMochiJSVersion } from '../utils';

export default async function handleBuild(src: string) {
  consola.log('');
  consola.start('Checking project...');

  src = path.resolve(process.cwd(), src);

  // gather all directories in src, as each module needs to have its own directory and own `index.ts`.

  await directoryExists(src);
  await retrieveMochiJSVersion(src);
  const modulesDirents = await getModulesDirectories(src);

  const buildPaths = modulesDirents
    .map((e) => path.join(e.path, e.name, 'index.ts'))
    .concat([path.join(src, 'index.ts')]);

  // Gather the tsconfig.json if present, else use default one. We should not emit for building.
  // Bundle should handle compiling.

  const tsConfigFilePath = path.resolve(src, 'tsconfig.json');
  const configOptions: ts.CompilerOptions = await readFile(tsConfigFilePath, { encoding: 'utf8' })
    .then((tsConfigString) => ts.parseConfigFileTextToJson(tsConfigFilePath, tsConfigString))
    .then((config) => {
      if (config.config.compilerOptions) return config.config.compilerOptions as ts.CompilerOptions;
      throw new Error(`failed to parse ${tsConfigFilePath}.\n\n${config.error}`);
    })
    .catch((e) => {
      consola.warn(`an error occurred parsing tsconfig.json, using default tsconfig instead.\n    error: ${e}`);
      return {} as ts.CompilerOptions;
    })
    .then((o) => {
      // We do not want to emit any files from ts to js.
      o.noEmit = true;
      o.strict = true;
      o.esModuleInterop = true;
      o.moduleResolution = ts.ModuleResolutionKind.NodeJs;
      o.isolatedModules = true;
      return o;
    });

  const program = ts.createProgram(buildPaths, configOptions);
  const emitResult = program.emit();

  let hasErrors = false;

  const allDiagnostics = ts.getPreEmitDiagnostics(program).concat(emitResult.diagnostics);
  if (allDiagnostics.length > 0) {
    consola.log('');
    allDiagnostics.forEach((diagnostic) => {
      const message = ts.flattenDiagnosticMessageText(diagnostic.messageText, '\n');
      let fileNameAndPosition: string;

      if (diagnostic.file) {
        const { line, character } = ts.getLineAndCharacterOfPosition(diagnostic.file, diagnostic.start!);
        const relativePath = path.relative(src!, diagnostic.file.fileName);
        fileNameAndPosition = `${relativePath}:${line + 1}:${character + 1} - `;
      } else {
        fileNameAndPosition = '';
      }

      switch (diagnostic.category) {
        case ts.DiagnosticCategory.Error:
          consola.fail(`${fileNameAndPosition}${message}`);
          hasErrors = true;
          break;
        case ts.DiagnosticCategory.Warning:
          consola.warn(`${fileNameAndPosition}${message}`);
          break;
        case ts.DiagnosticCategory.Suggestion:
          consola.info(`${fileNameAndPosition}${message}`);
          break;
        case ts.DiagnosticCategory.Message:
          consola.log(`${fileNameAndPosition}${message}`);
          break;
      }
    });
  }

  if (hasErrors) {
    throw new Error('This project has errors. Please resolve them before bundling.');
  } else {
    consola.log('');
    consola.success('Project built successfully!');
  }
}
