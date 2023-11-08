import { programForTestingUnknownOption } from './helpers';
import { expect } from 'chai';

describe('mochi-cli', () => {
  describe('watch', () => {
    it('should throw error if command receives unknown option', function () {
      expect(programForTestingUnknownOption('watch'), 'expected an error throws for unknown option').to.throw(
        "error: unknown option '--error'",
      );
    });
  });
});
