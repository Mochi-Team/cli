import { programForTestingUnknownOption } from './helpers';
import { expect } from 'chai';

describe('mochi-cli', () => {
  describe('build', () => {
    it('should throw error if command receives unknown option', function () {
      expect(programForTestingUnknownOption('build'), 'expected an error throws for unknown option').to.throw(
        "error: unknown option '--error'",
      );
    });
  });
});
