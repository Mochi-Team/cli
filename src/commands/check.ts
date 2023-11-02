import * as path from 'path';
import * as esbuild from 'esbuild';
import { buildOptions } from '../utils';

export default async function handleCheck(dir: string) {
  dir = path.resolve(process.cwd(), dir);

  await esbuild.build(await buildOptions(dir));
}
