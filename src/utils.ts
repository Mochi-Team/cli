import * as path from "path";

import consola from "consola";
import { Dirent } from "fs";
import { readFile, readdir, stat } from "fs/promises";

export const directoryExists = async (src: string) => await stat(src)
    .then(a => !a.isDirectory() ? Promise.reject(new Error(`${src} is not a valid directory.`)) : undefined);

export const getModulesDirectories = async (src: string, log: boolean = true) => {
    let directories = await readdir(path.join(src, 'src'), { withFileTypes: true })

    return await Promise.all(
        directories.map(f =>
            stat(path.join(f.path, f.name, 'index.ts'))
                .then(s => {
                    if (s.isFile()) {
                        return f;
                    }
                    throw new Error('index.ts does not exist for file.');
                })
                .then(_ => f)
                .catch(_ => {
                    if (log) consola.warn(`${path.relative(src, path.join(f.path, f.name))} does not contain an index.ts. Skipping..`);
                    return undefined;
                })
        )
    )
        .then(l => l.filter((d): d is Dirent => !!d));
}

export const retrieveMochiJSVersion = async (src: string) => await readFile(path.join(src, 'node_modules', '@mochi', 'js', 'package.json'), { encoding: 'utf-8' })
    .then(value => JSON.parse(value))
    .then((json: { version?: string; }) => {
        if (json?.version) return json.version;
        throw Error(`failed to find @mochi/js version in ${path.join(src, 'node_modules', '@mochi', 'js', 'package.json')}`);
    });