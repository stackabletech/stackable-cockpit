/* eslint-disable unicorn/prevent-abbreviations */
import { assert, describe, it } from 'vitest';
import { Err, None, Ok, Some } from '../src/types';

describe('option tests', () => {
  it('option isSome', () => {
    const option = Some('foo');

    assert.isTrue(option.isSome());
    assert.isFalse(option.isNone());
  });

  it('option isSomeAnd', () => {
    const option = Some('foo');

    assert.isTrue(option.isSomeAnd((value) => value.length === 3));
    assert.isFalse(option.isSomeAnd((value) => value.length === 2));
    assert.isFalse(option.isNone());
  });

  it('option isNone', () => {
    const option = None();

    assert.isFalse(option.isSome());
    assert.isTrue(option.isNone());
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

  it('option or', () => {
    const optionA = Some('foo');
    const optionAOr = optionA.or(Some('bar'));

    assert.deepEqual(optionAOr, Some('foo'));

    const optionB = None();
    const optionBOr = optionB.or(Some('bar'));

    assert.deepEqual(optionBOr, Some('bar'));
  });

  it('option orElse', () => {
    const optionA = Some('foo');
    const optionAOr = optionA.orElse(() => Some('bar'));

    assert.deepEqual(optionAOr, Some('foo'));

    const optionB = None();
    const optionBOr = optionB.orElse(() => Some('bar'));

    assert.deepEqual(optionBOr, Some('bar'));
  });

  it('option xor', () => {
    const optionA = Some('foo');
    const someA = optionA.xor(None());
    const noneA = optionA.xor(Some('bar'));

    assert.deepEqual(someA, Some('foo'));
    assert.deepEqual(noneA, None());

    const optionB = None();
    const noneB = optionB.xor(None());
    const someB = optionB.xor(Some('bar'));

    assert.deepEqual(noneB, None());
    assert.deepEqual(someB, Some('bar'));
  });

  it('option take', () => {
    const optionA = Some('foo');
    const valueA = optionA.take();

    assert.deepEqual(optionA, None());
    assert.deepEqual(valueA, Some('foo'));

    const optionB = None();
    const valueB = optionA.take();

    assert.deepEqual(optionB, None());
    assert.deepEqual(valueB, None());
  });

  it('option replace', () => {
    const optionA = Some('foo');
    const oldA = optionA.replace('bar');

    assert.deepEqual(optionA, Some('bar'));
    assert.deepEqual(oldA, Some('foo'));

    // FIXME (Techassi): Replacing on None is impossible
  });

  it('option contains', () => {
    const optionA = Some('foo');
    assert.isTrue(optionA.contains('foo'));

    const optionB = Some('foo');
    assert.isFalse(optionB.contains('bar'));
  });

  it('option zip', () => {
    const option = Some('foo');
    const zipped = option.zip(Some(3));
    assert.deepEqual(zipped, Some(['foo', 3]));

    const zippedNone = option.zip(None());
    assert.deepEqual(zippedNone, None());
  });

  it('option zipWith', () => {
    const option = Some('foo');
    const zipped = option.zipWith(Some(3), (str, num) => str.repeat(num));
    assert.deepEqual(zipped, Some('foofoofoo'));

    const zippedNone = option.zipWith(None(), () => 'bar');
    assert.deepEqual(zippedNone, None());
  });

  it('option unwrap', () => {
    const option = Some('foo');
    assert.equal(option.unwrap(), 'foo');

    const optionNone = None();
    assert.throw(() => optionNone.unwrap());
  });

  it('option unwrapOr', () => {
    const optionA = Some('foo');
    assert.equal(optionA.unwrapOr('bar'), 'foo');

    const optionB = None();
    assert.equal(optionB.unwrapOr(3), 3);
  });

  it('option unwrapOrElse', () => {
    const optionA = Some('foo');
    assert.equal(
      optionA.unwrapOrElse(() => 3),
      'foo',
    );

    const optionB = None();
    assert.equal(
      optionB.unwrapOrElse(() => 3),
      3,
    );
  });
});
