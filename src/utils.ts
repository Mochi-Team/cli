import * as path from 'path';

import vm from 'vm';
import consola from 'consola';
import { rm, readFile, writeFile, mkdir, readdir, stat } from 'fs/promises';
import ejs from 'ejs';
import ts from 'typescript';
import esbuild from 'esbuild';

const MOCHI_JS_NAME = '@mochiapp/js';

type ModulePath = {
  name: string;
  path: string;
};

const getModulesDirectories = async (basedir: string, log: boolean = true): Promise<ModulePath[]> => {
  consola.log('');
  consola.info(`Verifying ${basedir}`);
  const directories = await readdir(path.join(basedir, 'src'));

  const allModules: ModulePath[] = await Promise.all(
    directories.map((item) => {
      const itemPath = path.join(basedir, 'src', item);
      const itemIndexPath = path.join(itemPath, 'index.ts');
      return stat(itemIndexPath)
        .then((s) => {
          if (s.isFile()) {
            if (log) consola.log(`   \x1b[32m\u21B3\x1b[0m ${itemIndexPath} - module found!`);
            return {
              name: item,
              path: itemPath,
            } as ModulePath;
          } else {
            throw new Error(''); // stub
          }
        })
        .catch(() => {
          if (log) consola.log(`   \x1b[33m\u26A0\x1b[0m ${itemPath} - does not contain an index.ts file, skipping..`);
          return undefined;
        });
    }),
  ).then((l) => l.filter((i): i is ModulePath => !!i));

  consola.log('');
  consola.info(`Found ${allModules.length} module${allModules.length == 1 ? '' : 's'}.`);

  return allModules;
};

export const retrieveMochiJSVersion = async (src: string) => {
  consola.log('');

  const version = await readFile(path.resolve(src, 'node_modules', MOCHI_JS_NAME, 'package.json'), {
    encoding: 'utf-8',
  })
    .catch((e) => {
      throw Error(`Failed to retrieve ${MOCHI_JS_NAME} version - ${e}`);
    })
    .then((value) => JSON.parse(value))
    .then((json?: { version?: string }) => {
      if (json?.version) return json.version;
      throw Error(`could not parse ${MOCHI_JS_NAME} version in node_modules/${MOCHI_JS_NAME}/package.json`);
    });

  consola.info(`found ${MOCHI_JS_NAME} version: ${version}`);
  return version;
};

export function toKebabCase(value: string): string {
  return value
    .toLowerCase()
    .replace(/([a-z])([A-Z])/g, '$1-$2')
    .replace(/[\s_]+/g, '-');
}

// TODO: Add JS support

// const isTypeScriptProject = async (dir: string) => {
//   return (await stat(path.resolve(dir, 'tsconfig.json'))).isFile();
// }

const REPOSITORY_GEN_NAME = '__repository';

export type BundleOption = {
  outdir: string;
  bundle?: boolean;
  site?: boolean;
  serve?: boolean;
};

export const buildOptions = async (basedir: string, outOptions?: BundleOption, typecheck: boolean = true) => {
  const mochiJSVesion = await retrieveMochiJSVersion(basedir);
  const modulesDirs = await getModulesDirectories(basedir);

  const entryPoints = modulesDirs.map((d) => {
    return {
      in: path.resolve(d.path, 'index.ts'),
      out: d.name,
    };
  });

  const plugins: esbuild.Plugin[] = [];

  // FIXME: right now typecheck is ignored for fast development.
  // Maybe cached last modified file to avoid slowdowns?
  if (typecheck && !(outOptions?.serve ?? false)) plugins.push(pluginTypeCheck(basedir));
  if (outOptions) plugins.push(pluginBundle(outOptions, mochiJSVesion));

  return <esbuild.BuildOptions>{
    entryPoints: entryPoints.concat([{ in: path.resolve(basedir, 'index.ts'), out: REPOSITORY_GEN_NAME }]),
    allowOverwrite: true,
    bundle: outOptions?.bundle ?? true,
    globalName: 'source',
    outdir: path.resolve(outOptions?.outdir ?? path.resolve(basedir, 'dist'), 'modules'),
    write: false,
    plugins: plugins,
    minify: !(outOptions?.serve ?? false),
    tsconfig: path.resolve(basedir, 'tsconfig.json'),
    absWorkingDir: basedir,
  };
};

export const pluginTypeCheck = (src: string) =>
  <esbuild.Plugin>{
    name: 'type-check',
    setup(build) {
      build.onStart(async () => {
        consola.log('');
        consola.start('Checking project...');

        const tsConfigFilePath = build.initialOptions.tsconfig!;
        const configOptions: ts.CompilerOptions = await readFile(tsConfigFilePath, { encoding: 'utf8' })
          .then((tsConfigString) => ts.parseConfigFileTextToJson(tsConfigFilePath, tsConfigString))
          .then((tsConfigJSON) => ts.parseJsonConfigFileContent(tsConfigJSON.config, ts.sys, src))
          .then((object) => {
            if (object.options) return object.options as ts.CompilerOptions;
            throw new Error(`failed to parse ${tsConfigFilePath}.`);
          })
          .catch((e) => {
            consola.warn(`an error occurred parsing tsconfig.json, using default tsconfig instead. error log: ${e}`);
            return {} as ts.CompilerOptions;
          })
          .then((o) => {
            // We do not want to emit any files from ts to js.
            o.noEmit = true;
            return o;
          });

        const entryPoints = build.initialOptions.entryPoints as { in: string; out: string }[];
        const program = ts.createProgram(
          entryPoints.map((i) => i.in),
          configOptions,
        );

        const emitResult = program.emit();
        const allDiagnostics = ts.getPreEmitDiagnostics(program).concat(emitResult.diagnostics);
        if (allDiagnostics.length > 0) consola.log('');

        await logDiagnostics(src, allDiagnostics);

        if (allDiagnostics.findIndex((value) => value.category == ts.DiagnosticCategory.Error) !== -1) {
          throw new Error('This project has errors. Please resolve them before bundling.');
        }

        consola.success('Successfully checked project and reported 0 build erros!');
      });
    },
  };

export const pluginBundle = (options: BundleOption, mochiJSVersion: string) =>
  <esbuild.Plugin>{
    name: 'bundle',
    setup(build) {
      build.onEnd(async (result) => {
        if (result.errors.length > 0) {
          consola.fail('Failed to bundle due to previous errors.');
          return;
        }

        consola.log('');
        consola.start('Bundling repository...');

        const releases: any[] = [];
        let repositoryMetadata: any | undefined;

        const DEST_MODULES_PATH = path.resolve(options.outdir, 'modules');
        const DEST_MANIFEST_PATH = path.resolve(options.outdir, 'Manifest.json');

        for (const output of result.outputFiles as esbuild.OutputFile[]) {
          const fileNameWExt = path.basename(output.path);
          const fileExt = path.extname(fileNameWExt);
          const fileName = path.basename(fileNameWExt, fileExt);

          if (fileExt != '.js') continue;

          if (REPOSITORY_GEN_NAME === fileName) {
            const metadata = vm.runInNewContext(`${output.text}; source.default`);
            if (metadata) {
              repositoryMetadata = metadata;
            } else {
              throw new Error(
                `failed to retrieve metadata content for ${fileNameWExt}. Make sure this repository's index.ts exports \`RepoMetadata\` by default.`,
              );
            }
          } else {
            const metadata = vm.runInNewContext(`${output.text}; new source.default().metadata`);
            if (metadata) {
              metadata.id = metadata.id ?? `${toKebabCase(metadata.name)}`;
              metadata.file = `/modules/${fileNameWExt}`;
              metadata.meta = []; // TODO: gather if it has video, image, or source
              metadata.mochiJSVersion = mochiJSVersion;
              releases.push(metadata);
            } else {
              throw new Error(
                `failed to retrieve metadata content from ${fileNameWExt}. Make sure the module class is exported by default and extends \`SourceModule\` or \`TrackerModule\`.`,
              );
            }
          }
        }

        if (!repositoryMetadata) throw new Error(`failed to retrieve metadata content from repository's src/index.ts`);

        const manifest = {
          repository: repositoryMetadata,
          modules: releases,
        };

        // delete old modules and manifest in dest if present

        await rm(DEST_MODULES_PATH, { recursive: true, force: true });
        await rm(DEST_MANIFEST_PATH, { force: true });
        await mkdir(DEST_MODULES_PATH, { recursive: true });

        await writeFile(DEST_MANIFEST_PATH, JSON.stringify(manifest, undefined, 2));

        await Promise.all(
          result.outputFiles!.map((o) => {
            if (!o.path.includes(REPOSITORY_GEN_NAME)) {
              return writeFile(o.path, o.contents);
            }
            return undefined;
          }),
        );

        if (options.site) {
          consola.log('');
          consola.start(`Generating site for repository...`);
          const template = await readFile(path.resolve(__dirname, 'templates/site/index.ejs'), {
            encoding: 'utf-8',
          });
          const output = await ejs.render(template, manifest, { async: true });
          await writeFile(path.join(options.outdir, 'index.html'), output);
          consola.success(`Successfully generated site!\n`);
        }

        consola.success('Successfully finished bundling repository!');
      });
    },
  };

function transformDiagnostic(basedir: string, diagnostic: ts.Diagnostic): esbuild.PartialMessage {
  const message = ts.flattenDiagnosticMessageText(diagnostic.messageText, '\n');

  const { code, file, length, start } = diagnostic;

  if (!file)
    return {
      id: `TS${code}`,
      text: message,
    };

  if (!start || !length)
    return {
      id: `TS${code}`,
      text: message,
    };

  const { line, character } = file.getLineAndCharacterOfPosition(start);
  const lastLineInFile = file.getLineAndCharacterOfPosition(file.text.length).line;

  const lineStart = file.getPositionOfLineAndCharacter(line, 0);
  const lineEnd = line < lastLineInFile ? file.getPositionOfLineAndCharacter(line + 1, 0) : file.text.length;

  const lineText = file.text.slice(lineStart, lineEnd).trimEnd();
  const safeLength = character + length > lineEnd - lineStart ? lineEnd - lineStart - character : length;

  return {
    id: `TS${code}`,
    text: message,
    location: {
      file: path.relative(basedir, file.fileName),
      line: line + 1,
      column: character,
      length: safeLength,
      lineText: lineText,
    },
  };
}

async function logDiagnostics(basedir: string, diagnostics: ts.Diagnostic[]) {
  const errors = diagnostics
    .filter((d) => d.category == ts.DiagnosticCategory.Error)
    .map((d) => transformDiagnostic(basedir, d));

  const warnings = diagnostics
    .filter((d) => d.category != ts.DiagnosticCategory.Error)
    .map((d) => transformDiagnostic(basedir, d));

  if (errors.length > 0) {
    await esbuild
      .formatMessages(errors, {
        color: true,
        kind: 'error',
      })
      .then((o) => consola.log(o.join('\n')));
  }

  if (warnings.length > 0) {
    await esbuild
      .formatMessages(warnings, {
        color: true,
        kind: 'warning',
      })
      .then((o) => consola.log(o.join('\n')));
  }
}
