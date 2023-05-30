/* eslint-disable unicorn/prevent-abbreviations */
import { assert, describe, it } from 'vitest';
import { Err, None, Ok, Some } from '../src/types';

describe('option tests', () => {
  it('option isSome', () => {
    const option = Some('foo');

    assert.equal(option.isSome(), true);
    assert.equal(option.isNone(), false);
  });

  it('option isSomeAnd', () => {
    const option = Some('foo');

    assert.equal(
      option.isSomeAnd((value) => value.length === 3),
      true,
    );

    assert.equal(
      option.isSomeAnd((value) => value.length === 2),
      false,
    );

    assert.equal(option.isNone(), false);
  });

  it('option isNone', () => {
    const option = None();

    assert.equal(option.isSome(), false);
    assert.equal(option.isNone(), true);
  });

  it('option map', () => {
    const optionA = Some('foo');
    assert.deepEqual(
      optionA.map((value) => value.length),
      Some(3),
    );

    const optionB = None();
    assert.deepEqual(
      optionB.map(() => 3),
      None(),
    );
  });

  it('option mapOr', () => {
    const optionA = Some('foo');
    assert.equal(
      optionA.mapOr(2, (value) => value.length),
      3,
    );

    const optionB = None();
    assert.equal(
      optionB.mapOr(2, () => 3),
      2,
    );
  });

  it('option mapOrElse', () => {
    const optionA = Some('foo');
    assert.equal(
      optionA.mapOrElse(
        () => 2,
        (value) => value.length,
      ),
      3,
    );

    const optionB = None();
    assert.equal(
      optionB.mapOrElse(
        () => 2,
        () => 3,
      ),
      2,
    );
  });

  it('option okOr', () => {
    const optionA = Some('foo');
    assert.deepEqual(optionA.okOr('Error'), Ok('foo'));

    const optionB = None();
    assert.deepEqual(optionB.okOr('Error'), Err('Error'));
  });

  it('option okOrElse', () => {
    const optionA = Some('foo');
    assert.deepEqual(
      optionA.okOrElse(() => 'Error'),
      Ok('foo'),
    );

    const optionB = None();
    assert.deepEqual(
      optionB.okOrElse(() => 'Error'),
      Err('Error'),
    );
  });

  it('option and', () => {
    const optionA = Some('foo');
    assert.deepEqual(optionA.and(Some('bar')), Some('bar'));

    const optionB = None();
    assert.deepEqual(optionB.and(Some('var')), None());
  });

  it('option andThen', () => {
    const optionA = Some('foo');
    assert.deepEqual(
      optionA.andThen((value) => Some(value.length)),
      Some(3),
    );

    const optionB = Some('foo');
    assert.deepEqual(
      optionB.andThen(() => None()),
      None(),
    );

    const optionC = None();
    assert.deepEqual(
      optionC.andThen(() => Some(3)),
      None(),
    );
  });

  it('option filter', () => {
    const optionA = Some('foo');
    assert.deepEqual(
      optionA.filter((value) => value.length === 3),
      Some('foo'),
    );

    const optionB = Some('foo');
    assert.deepEqual(
      optionB.filter((value) => value.length === 2),
      None(),
    );

    const optionC = None();
    assert.deepEqual(
      optionC.filter(() => true),
      None(),
    );
  });
});
