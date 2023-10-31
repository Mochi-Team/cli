import * as path from "path";
import ts from 'typescript';
import { readFile } from 'fs/promises';
import { consola } from 'consola';
import JSON5 from 'json5';
import { directoryExists, getModulesDirectories, retrieveMochiJSVersion } from '../utils';

export default async function handleBuild(src?: string) {
  consola.start("Building project...");

  if (src == undefined) {
    src = process.cwd();
  }

  // gather all directories in src, as each module needs to have its own directory and own `index.ts`.

  await directoryExists(src);
  await retrieveMochiJSVersion(src);
  let modulesDirents = await getModulesDirectories(src);

  let buildPaths = modulesDirents.map(e => path.join(e.path, e.name, 'index.ts')).concat([path.join(src, 'index.ts')]);

  // Gather the tsconfig.json if present, else use default one. We should not emit for building.
  // Bundle should handle compiling.

  let tsconfigFile: ts.CompilerOptions = await readFile(path.join(src, 'tsconfig.json'), { encoding: 'utf8' })
    .then(o => JSON5.parse(o).compilerOptions)
    .catch(e => {
      consola.warn(`there was an error retrieving tsconfig.json, using default tsconfig instead.\n    reason: ${e}`)
      return {};
    })
    .then(o => {
      // We do not want to emit any files from ts to js.
      o.noEmit = true;
      o.strict = true;
      o.esModuleInterop = true;
      o.moduleResolution = ts.ModuleResolutionKind.NodeJs;
      o.isolatedModules = true;
      return o;
    });

  let program = ts.createProgram(buildPaths, tsconfigFile);
  let emitResult = program.emit();

  let allDiagnostics = ts
    .getPreEmitDiagnostics(program)
    .concat(emitResult.diagnostics);

  let hasErrors = false;

  allDiagnostics.forEach(diagnostic => {
    let message = ts.flattenDiagnosticMessageText(diagnostic.messageText, "\n");
    let fileNameAndPosition: string;

    if (diagnostic.file) {
      let { line, character } = ts.getLineAndCharacterOfPosition(diagnostic.file, diagnostic.start!);
      let relativePath = diagnostic.file.fileName.replace(src!, "");
      fileNameAndPosition = `${relativePath}:${line + 1}:${character + 1} - `;
    } else {
      fileNameAndPosition = '';
    }

    switch (diagnostic.category) {
      case ts.DiagnosticCategory.Error:
        consola.fail(`${fileNameAndPosition}${message}`)
        hasErrors = true;
        break;
      case ts.DiagnosticCategory.Warning:
        consola.warn(`${fileNameAndPosition}${message}`)
        break;
      case ts.DiagnosticCategory.Suggestion:
        consola.info(`${fileNameAndPosition}${message}`)
        break;
      case ts.DiagnosticCategory.Message:
        consola.log(`${fileNameAndPosition}${message}`)
        break;
    }
  });

  if (hasErrors) {
    throw new Error('This project has errors. Please resolve them before bundling.')
  } else {
    consola.success("Project built successfully!\n");
  }
}