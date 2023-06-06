/* eslint-disable unicorn/prevent-abbreviations */
import { ToString } from './utils';
import { Err, Ok, Result } from './result';

interface BaseOption<S> {
  /**
   * Returns true if the option is a `Some` value.
   *
   * @returns boolean
   */
  isSome(): this is Option<S>;

  /**
   * Returns true if the option is a `Some` and the value inside of it matches
   * a predicate.
   *
   * @param fn Predicate to match
   * @returns boolean
   */
  isSomeAnd(fn: (value: Readonly<S>) => boolean): this is Option<S>;

  /**
   * Returns true if the option is a `None` value.
   *
   * @returns boolean
   */
  isNone(): this is Option<never>;

  match<T, K>(matcher: {
    some: (value: Readonly<S>) => T;
    none: () => K;
  }): T | K;

  map<T>(fn: (value: Readonly<S>) => T): Option<T>;

  mapOr<T>(defaultValue: T, fn: (value: Readonly<S>) => T): T;

  mapOrElse<T>(defaultFn: () => T, fn: (value: Readonly<S>) => T): T;

  okOr<E extends ToString>(error: E): Result<S, E>;

  okOrElse<E extends ToString>(err: () => E): Result<S, E>;

  and<T>(other: Option<T>): Option<T>;

  andThen<T>(fn: (value: Readonly<S>) => Option<T>): Option<T>;

  filter(predicate: (value: Readonly<S>) => boolean): Option<S>;

  or<T>(other: Option<S | T>): Option<S | T>;

  orElse<T>(fn: () => Option<S | T>): Option<S | T>;

  xor<T>(other: Option<S | T>): Option<S | T>;

  contains(value: S): boolean;

  zip<T>(other: Option<T>): Option<[S, T]>;

  zipWith<T, K>(other: Option<T>, fn: (self: S, other: T) => K): Option<K>;

  unwrap(): S;

  unwrapOr<T>(defaultValue: S | T): S | T;

  unwrapOrElse<T>(fn: () => S | T): S | T;

  expect(message: string): S;
}

class NoneImpl implements BaseOption<never> {
  isSome(): this is Option<never> {
    return false;
  }

  isSomeAnd(fn: (value: never) => boolean): this is Option<never> {
    return this.match({
      some: (value) => fn(value),
      none: () => false,
    });
  }

  isNone(): this is Option<never> {
    return !this.isSome();
  }

  match<T, K>(matcher: { some: (value: never) => T; none: () => K }): T | K {
    return matcher.none();
  }

  map<T>(fn: (value: never) => T): Option<T> {
    return this.match({
      some: (value) => Some(fn(value)),
      none: () => None,
    });
  }

  mapOr<T>(defaultValue: T, fn: (value: never) => T): T {
    return this.match({
      some: (value) => fn(value),
      none: () => defaultValue,
    });
  }

  mapOrElse<T>(defaultFn: () => T, fn: (value: never) => T): T {
    return this.match({
      some: (value) => fn(value),
      none: defaultFn,
    });
  }

  okOr<E extends ToString>(error: E): Result<never, E> {
    return this.match({
      some: (value) => Ok(value),
      none: () => Err(error),
    });
  }

  okOrElse<E extends ToString>(err: () => E): Result<never, E> {
    return this.match({
      some: (value) => Ok(value),
      none: () => Err(err()),
    });
  }

  and<T>(other: Option<T>): Option<T> {
    return this.match({
      some: () => other,
      none: () => None,
    });
  }

  andThen<T>(fn: (value: never) => Option<T>): Option<T> {
    return this.match({
      some: (value) => fn(value),
      none: () => None,
    });
  }

  filter(predicate: (value: never) => boolean): Option<never> {
    return this.match({
      some: (value) => (predicate(value) ? Some(value) : None),
      none: () => None,
    });
  }

  or<T>(other: Option<T>): Option<T> {
    return this.match({
      some: (value) => Some(value),
      none: () => other,
    });
  }

  orElse<T>(fn: () => Option<T>): Option<T> {
    return this.match({
      some: (value) => Some(value),
      none: fn,
    });
  }

  xor<T>(other: Option<T>): Option<T> {
    if (this.isSome() && other.isNone()) {
      return this;
    } else if (this.isNone() && other.isSome()) {
      return other;
    } else {
      return None;
    }
  }

  contains(value: never): boolean {
    return this.match({
      some: (inner) => inner === value,
      none: () => false,
    });
  }

  zip<T>(other: Option<T>): Option<[never, T]> {
    return this.isSome() && other.isSome()
      ? Some([this.unwrap(), other.unwrap()])
      : None;
  }

  zipWith<T, K>(other: Option<T>, fn: (self: never, other: T) => K): Option<K> {
    return this.isSome() && other.isSome()
      ? Some(fn(this.unwrap(), other.unwrap()))
      : None;
  }

  unwrap(): never {
    return this.expect('called `Option::unwrap()` on a `None` value');
  }

  unwrapOr<T>(defaultValue: T): T {
    return this.match({
      some: (value) => value,
      none: () => defaultValue,
    });
  }

  unwrapOrElse<T>(fn: () => T): T {
    return this.match({
      some: (value) => value,
      none: fn,
    });
  }

  expect(message: string): never {
    return this.match({
      some: (value) => value,
      none: () => {
        throw new Error(message);
      },
    });
  }
}

export const None = new NoneImpl();
export type None = NoneImpl;
Object.freeze(None);

class SomeImpl<S> implements BaseOption<S> {
  private value!: S;

  constructor(value: S) {
    this.value = value;
  }

  isSome(): this is Option<S> {
    return true;
  }

  isSomeAnd(op: (value: Readonly<S>) => boolean): this is Option<S> {
    return this.isSome() && op(this.value);
  }

  isNone(): this is Option<never> {
    return !this.isSome();
  }

  match<T, K>(matcher: {
    some: (value: Readonly<S>) => T;
    none: () => K;
  }): T | K {
    // eslint-disable-next-line unicorn/no-array-callback-reference
    return matcher.some(this.value);
  }

  map<T>(fn: (value: Readonly<S>) => T): Option<T> {
    return this.match({
      some: (value) => Some(fn(value)),
      none: () => None,
    });
  }

  mapOr<T>(defaultValue: T, fn: (value: Readonly<S>) => T): T {
    return this.match({
      some: (value) => fn(value),
      none: () => defaultValue,
    });
  }

  mapOrElse<T>(defaultFn: () => T, fn: (value: Readonly<S>) => T): T {
    return this.match({
      some: (value) => fn(value),
      none: defaultFn,
    });
  }

  okOr<E extends ToString>(error: E): Result<S, E> {
    return this.match({
      some: (value) => Ok(value),
      none: () => Err(error),
    });
  }

  okOrElse<E extends ToString>(err: () => E): Result<S, E> {
    return this.match({
      some: (value) => Ok(value),
      none: () => Err(err()),
    });
  }

  and<T>(other: Option<T>): Option<T> {
    return this.match({
      some: () => other,
      none: () => None,
    });
  }

  andThen<T>(fn: (value: Readonly<S>) => Option<T>): Option<T> {
    return this.match({
      some: (value) => fn(value),
      none: () => None,
    });
  }

  filter(predicate: (value: Readonly<S>) => boolean): Option<S> {
    return this.match({
      some: (value) => (predicate(value) ? Some(value) : None),
      none: () => None,
    });
  }

  or<T>(other: Option<S | T>): Option<S | T> {
    return this.match({
      some: (value) => Some(value),
      none: () => other,
    });
  }

  orElse<T>(fn: () => Option<S | T>): Option<S | T> {
    return this.match({
      some: (value) => Some(value),
      none: fn,
    });
  }

  xor<T>(other: Option<S | T>): Option<S | T> {
    if (this.isSome() && other.isNone()) {
      return this;
    } else if (this.isNone() && other.isSome()) {
      return other;
    } else {
      return None;
    }
  }

  contains(value: S): boolean {
    return this.match({
      some: (inner) => inner === value,
      none: () => false,
    });
  }

  zip<T>(other: Option<T>): Option<[S, T]> {
    return this.isSome() && other.isSome()
      ? Some([this.unwrap(), other.unwrap()])
      : None;
  }

  zipWith<T, K>(other: Option<T>, fn: (self: S, other: T) => K): Option<K> {
    return this.isSome() && other.isSome()
      ? Some(fn(this.unwrap(), other.unwrap()))
      : None;
  }

  unwrap(): S {
    return this.expect('called `Option::unwrap()` on a `None` value');
  }

  unwrapOr<T>(defaultValue: S | T): S | T {
    return this.match({
      some: (value) => value,
      none: () => defaultValue,
    });
  }

  unwrapOrElse<T>(fn: () => S | T): S | T {
    return this.match({
      some: (value) => value,
      none: fn,
    });
  }

  expect(message: string): S {
    return this.match({
      some: (value) => value,
      none: () => {
        throw new Error(message);
      },
    });
  }
}

export const Some = <T>(value: T) => new SomeImpl(value);
export type Some<S> = SomeImpl<S>;

export type Option<S> = Some<S> | None;
