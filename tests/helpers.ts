import { Command } from 'commander';

export const generateProgram = (overrideExit?: boolean) => {
  const program = new Command();

  if (overrideExit) {
    program.exitOverride();
  }

  program.exitOverride();

  return program;
};

export const programForTestingUnknownOption = (command: string) => {
  const program = new Command();
  program
    .exitOverride()
    .command(command)
    .configureOutput({
      writeErr: () => {
        /* noop */
      },
    });

  return program.parse.bind(program, ['node', 'mochi-cli', command, '--error']);
};
