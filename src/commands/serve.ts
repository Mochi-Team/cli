import { consola } from 'consola';
import path from 'path';

export default async function handleServe(src: string, dest: string, site: boolean, watch: boolean) {
  src = path.resolve(process.cwd(), src);
  dest = path.resolve(process.cwd(), dest);

  consola.info(`serve w/ source: ${src}, dest: ${dest}, site: ${site}, watch: ${watch}`);
}
