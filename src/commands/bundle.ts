import * as path from 'path';
// import { consola } from 'consola';
import esbuild from 'esbuild';
import { buildOptions } from '../utils';

export default async function handleBundle(dir: string, dest: string, site: boolean) {
  dir = path.resolve(process.cwd(), dir);
  dest = path.resolve(process.cwd(), dest);

  await esbuild.build(await buildOptions(dir, { outdir: dest, site: site }));
}
