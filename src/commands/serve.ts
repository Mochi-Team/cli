import { consola } from 'consola';
import esbuild from 'esbuild';
import path from 'path';
import { buildOptions } from '../utils';
import os from 'node:os';

export default async function handleServe(
  dir: string,
  dest: string,
  site: boolean,
  watch: boolean,
  preferredPort: string,
) {
  dir = path.resolve(process.cwd(), dir);
  dest = path.resolve(process.cwd(), dest);

  const options = await buildOptions(dir, { outdir: dest, site: site, serve: true });
  const ctx = await esbuild.context(options);

  if (watch) {
    consola.log('');
    consola.start('Observing directory for changes...');
    await ctx.watch();
  }

  const { host, port } = await ctx.serve({
    port: parseInt(preferredPort),
    servedir: dest,
    onRequest: (args) => {
      consola.info(`received a ${args.method} request for ${args.path}.`);
    },
  });

  consola.log('');
  consola.success(`Started local server at http://${host}:${port}, lan: http://${retrieveLocalIP() ?? host}:${port}`);
}

const retrieveLocalIP = () => {
  const ifaces = os.networkInterfaces();

  for (const key in ifaces) {
    const iface = ifaces[key];

    for (const net of iface ?? []) {
      if (net.family == 'IPv4' && !net.internal) {
        return net.address;
      }
    }
  }
  return undefined;
};
