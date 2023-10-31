import * as path from 'path';
import vm from 'vm';
import { consola } from 'consola';
import handleBuild from './check';
import esbuild from 'esbuild';
import { getModulesDirectories, retrieveMochiJSVersion } from '../utils';
import { rm, readFile, writeFile } from 'fs/promises';
import * as ejs from 'ejs';

export default async function handleBundle(src?: string, dest?: string, site: boolean = false) {
    src = src ?? process.cwd();
    dest = dest ?? path.join(src, 'dist');

    const DEST_MODULES_PATH = path.join(dest, 'modules');

    await handleBuild(src);

    // Retrieve @mochi/js version from node_modules.

    let mochiJSVersion: string = await retrieveMochiJSVersion(src);

    // delete old modules in dest if present

    await rm(DEST_MODULES_PATH, { recursive: true, force: true });

    const modulesDirs = await getModulesDirectories(src, false);

    const entryPoints = modulesDirs.map(d => {
        return {
            in: path.join(d.path, d.name, 'index.ts'),
            out: d.name
        }
    });

    const repositoryPoint = {
        in: path.join(src, 'index.ts'),
        out: '__repository'
    };

    consola.start('Starting bundling repository...\n');

    // generate bundled files

    await esbuild.build({
        entryPoints: entryPoints.concat([repositoryPoint]),
        bundle: true,
        globalName: 'source',
        outdir: DEST_MODULES_PATH
    });

    // Read repository.js metadata

    const repoIndexJSPath = path.join(DEST_MODULES_PATH, `${repositoryPoint.out}.js`);

    const metadata = await readFile(repoIndexJSPath)
        .then(f => vm.runInNewContext(`${f}; source.default`))
        .then(v => {
            if (v) return v;
            throw new Error(`${path.join(src!, 'index.ts')} does not have repository metadata exported as default.`)
        })
        .finally(async () => await rm(repoIndexJSPath, { force: true }));

    // Load js files and gather module's metadata

    const releases: any[] = [];

    let compiledSourcesString = await Promise.all(
        entryPoints
        .map(o => readFile(path.join(DEST_MODULES_PATH, `${o.out}.js`), { encoding: 'utf-8' })
                .then(js => [o, js] as [{ in: string, out: string }, string])
            )
    );

    compiledSourcesString.forEach(o => {
        const moduleMetadata = vm.runInNewContext(`${o[1]}; new source.default().metadata`);
        if (moduleMetadata) {
            moduleMetadata.id = `${toKebabCase(moduleMetadata.name)}`;
            moduleMetadata.file = `/modules/${o[0].out}.js`;
            moduleMetadata.meta = []; // TODO: gather if it has video, image, or source
            moduleMetadata.mochiJSVersion = mochiJSVersion;
            releases.push(moduleMetadata);    
        } else {
            throw new Error(`failed to retrieve metadata content for ${o[0].in}`);
        }
    });

    const manifest = {
        modules: releases,
        repository: metadata
    }

    await writeFile(path.join(dest, 'Manifest.json'), JSON.stringify(manifest))

    consola.success(`Successfully bundled repository as ${dest}\n`);

    if (site) {
        consola.start(`Generating site for repository...`);
        const template = await readFile(path.resolve(__dirname, '..', 'templates/site/index.ejs'), { encoding: 'utf-8' });
        let output = await ejs.render(template, manifest, { async: true });

        await writeFile(path.join(dest, 'index.html'), output);

        consola.start(`Successfully generated site!`);
    }
}

function toKebabCase(value: string): string {
    return value
        .toLowerCase()
        .replace(/([a-z])([A-Z])/g, "$1-$2")
        .replace(/[\s_]+/g, '-');
}