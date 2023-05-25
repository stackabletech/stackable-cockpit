/* eslint-disable unicorn/prevent-abbreviations */
import { assert, describe, it } from 'vitest';

import { Err, None, Ok, Some } from '../src/types';

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

  it('result ok', () => {
    const result = Ok('foo');

    assert.deepEqual(result.ok(), Some('foo'));
    assert.deepEqual(result.err(), None());
  });

  it('result map', () => {
    const result = Ok('foo');
    const mapped = result.map((value) => value.length);

    assert.equal(mapped.unwrap(), 3);
  });

  it('result isErr', () => {
    const result = Err('foo');

    assert.equal(result.isErr(), true);
    assert.equal(result.isOk(), false);
  });

  it('result isErrAnd', () => {
    const result = Err('foo');

    assert.equal(
      result.isErrAnd((error) => error.length === 3),
      true,
    );
    assert.equal(
      result.isErrAnd((error) => error.length === 2),
      false,
    );
  });

  it('result err', () => {
    const result = Err('foo');

    assert.deepEqual(result.err(), Some('foo'));
    assert.deepEqual(result.ok(), None());
  });
});
