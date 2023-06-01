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

  it('result map', () => {
    const result = Ok('foo');
    const mapped = result.map((value) => value.length);

    assert.isTrue(result.isOk());
    assert.isTrue(mapped.isOk());

    assert.equal(result.unwrap(), 'foo');
    assert.equal(mapped.unwrap(), 3);
  });

  it('result mapOr', () => {
    const resultA = Ok('foo');
    const mappedA = resultA.mapOr(2, (value) => value.length);

    assert.isTrue(resultA.isOk());
    assert.equal(mappedA, 3);

    const resultB = Err('bar');
    const mappedB = resultB.mapOr(2, () => 3);

    assert.isTrue(resultB.isErr());
    assert.equal(mappedB, 2);
  });

  it('result mapOrElse', () => {
    const resultA = Ok('foo');
    const mappedA = resultA.mapOrElse(
      () => 2,
      (value) => value.length,
    );

    assert.isTrue(resultA.isOk());
    assert.equal(mappedA, 3);

    const resultB = Err('bar');
    const mappedB = resultB.mapOrElse(
      (err) => err.length,
      () => 2,
    );

    assert.isTrue(resultB.isErr());
    assert.equal(mappedB, 3);
  });

  it('result mapErr', () => {
    const resultA = Ok('foo');
    assert.deepEqual(
      resultA.mapErr(() => 'bar'),
      Ok('foo'),
    );

    const resultB = Err('foo');
    assert.deepEqual(
      resultB.mapErr((err) => err.length),
      Err(3),
    );
  });
});
