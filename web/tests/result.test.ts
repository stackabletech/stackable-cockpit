/* eslint-disable unicorn/prevent-abbreviations */
import { assert, describe, it } from 'vitest';

import { Err, Ok } from '../src/types';

describe('result tests', () => {
  it('result isOk', () => {
    const result = Ok('foo');

    assert.equal(result.isOk(), true);
    assert.equal(result.isErr(), false);
  });

  it('result isOkAnd', () => {
    const result = Ok('foo');

    assert.equal(
      result.isOkAnd((value) => value.length === 3),
      true,
    );
    assert.equal(
      result.isOkAnd((value) => value.length === 2),
      false,
    );
  });

  it('result isErr', () => {
    const result = Err('foo');

    assert.equal(result.isErr(), true);
    assert.equal(result.isOk(), false);
  });
});
