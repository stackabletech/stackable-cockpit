import { ToString } from './utils';
import { Err, Ok, Result } from './result';

interface BaseOption<S> {
  /**
   * Returns true if the option is a `Some` value.
   *
   * @returns boolean
   */
  isSome(): this is Some<S>;

  /**
   * Returns true if the option is a `Some` and the value inside of it matches
   * a predicate.
   *
   * @param fn Predicate to match
   * @returns boolean
   */
  isSomeAnd(fn: (value: Readonly<S>) => boolean): this is Some<S>;

  /**
   * Returns true if the option is a `None` value.
   *
   * @returns boolean
   */
  isNone(): this is None;

  match<T>(matcher: { some: (value: Readonly<S>) => T; none: () => T }): T;

  /**
   * Maps an `Option<S>` to `Option<T>` by applying a function to a contained
   * value (if `Some`) or returns `None` (if `None`).
   *
   * @param fn Mapping function.
   * @returns The mapped option.
   */
  map<T>(fn: (value: Readonly<S>) => T): Option<T>;

  /**
   * Returns the provided default result (if none), or applies a function to
   * the contained value (if any).
   *
   * @param defaultValue The default value if `None`.
   * @param fn The mapping function.
   */
  mapOr<T>(defaultValue: T, fn: (value: Readonly<S>) => T): T;

  /**
   * Computes a default function result (if none), or applies a different
   * function to the contained value (if any).
   *
   * @param defaultFn Closure computing the default value if `None`.
   * @param fn The mapping function.
   */
  mapOrElse<T>(defaultFn: () => T, fn: (value: Readonly<S>) => T): T;

  /**
   * Transforms the `Option<S>` into a `Result<S, E>`, mapping `Some(v)` to
   * `Ok(v)` and `None` to `Err(err)`.
   *
   * @param error The error value if `None`.
   * @returns The transformed result.
   */
  okOr<E extends ToString>(error: E): Result<S, E>;

  /**
   * Transforms the `Option<S>` into a `Result<S, E>`, mapping `Some(v)` to
   * `Ok(v)` and `None` to `Err(err())`.
   *
   * @param err Closure to compute the error value if `None`.
   * @returns The transformed result.
   */
  okOrElse<E extends ToString>(err: () => E): Result<S, E>;

  /**
   * Returns `None` if the option is `None`, otherwise returns `other`.
   *
   * @param other The other option if `Some`.
   * @returns Either `None` or the `other` option.
   */
  and<T>(other: Option<T>): Option<T>;

  /**
   * Returns `None` if the option is `None`, otherwise calls `fn` with the
   * wrapped value and returns the result.
   *
   * @param fn The function which produces the other option if `Some`.
   * @returns Either `None` or the computed other option.
   */
  andThen<T>(fn: (value: Readonly<S>) => Option<T>): Option<T>;

  /**
   * Returns `None` if the option is `None`, otherwise calls `predicate` with
   * the wrapped value and returns:
   *
   * - `Some(t)` if predicate returns true (where t is the wrapped value), and
   * - `None` if predicate returns false.
   *
   * @param predicate The filter function.
   * @returns `Some` if the filter function returned true, otherwise `None`.
   */
  filter(predicate: (value: Readonly<S>) => boolean): Option<S>;

  /**
   * Returns the option if it contains a value, otherwise returns `other`.
   *
   * @param other The `other` option if `None`.
   * @returns Either this or `other`.
   */
  or<T>(other: Option<S | T>): Option<S | T>;

  /**
   * Returns the option if it contains a value, otherwise calls `fn` and returns
   * the result.
   *
   * @param fn Function to compute the other option.
   * @returns Either this or the other computed option.
   */
  orElse<T>(fn: () => Option<S | T>): Option<S | T>;

  /**
   * Returns `Some` if exactly one of self, `other` is `Some`, otherwise returns
   * `None`.
   *
   * @param other The other option.
   */
  xor<T>(other: Option<S | T>): Option<S | T>;

  /**
   * Returns true if the option is a `Some` value containing the given `value`.
   *
   * @param value The value to match `Some(value)`.
   * @returns boolean
   */
  contains(value: S): boolean;

  /**
   * Zips `self` with another `Option`.
   *
   * If `self` is `Some(s)` and other is `Some(o)`, this method returns
   * `Some([s, o])`. Otherwise, `None` is returned.
   *
   * @param other The other option to zip with.
   * @returns The zipped option or `None`.
   */
  zip<T>(other: Option<T>): Option<[S, T]>;

  /**
   * Zips `self` and another `Option` with function `fn`.
   *
   * If `self` is `Some(s)` and other is `Some(o)`, this method returns
   * `Some(fn(s, o))`. Otherwise, `None` is returned.
   *
   * @param other The other option to zip with.
   * @param fn Function which get's applied to the zipped values.
   * @returns The zipped and transformed option or `None`.
   */
  zipWith<T, K>(other: Option<T>, fn: (self: S, other: T) => K): Option<K>;

  /**
   * Returns the contained `Some` value.
   *
   * Because this function may throw, its use is generally discouraged. Instead,
   * prefer to use pattern matching and handle the `None` case explicitly, or
   * call `unwrap_or` or `unwrap_or_else`.
   *
   * @returns The contained `Some` value.
   *
   * @throws Throws if the self value equals `None`.
   */
  unwrap(): S;

  /**
   * Returns the contained `Some` value or a provided default.
   *
   * @param defaultValue Default value returned when `None`.
   * @returns Either contained `Some` value of `defaultValue`.
   */
  unwrapOr<T>(defaultValue: S | T): S | T;

  /**
   * Returns the contained `Some` value or computes it from a closure.
   *
   * @param fn Closure to compute default value.
   */
  unwrapOrElse<T>(fn: () => S | T): S | T;

  /**
   * Returns the contained Some value
   *
   * ### Recommended Message Style
   *
   * We recommend that `expect` messages are used to describe the reason you
   * expect the `Option` should be `Some`.
   *
   * **Hint:** If you’re having trouble remembering how to phrase expect error
   * messages remember to focus on the word “should” as in “env variable should
   * be set by blah” or “the given binary should be available and executable by
   * the current user”.
   *
   * @param message Error message.
   * @returns The contained `Some` value.
   *
   * @throws Throws if the value is a `None` with a custom panic message
   * provided by `msg`.
   */
  expect(message: string): S;
}

class NoneImpl implements BaseOption<never> {
  isSome(): this is Some<never> {
    return false;
  }

  isSomeAnd(_fn: (value: never) => boolean): this is Some<never> {
    return false;
  }

  isNone(): this is None {
    return true;
  }

  match<T>(matcher: { none: () => T }): T {
    return matcher.none();
  }

  map<T>(_fn: (value: never) => T): None {
    return None;
  }

  mapOr<T>(defaultValue: T, _fn: (value: never) => T): T {
    return defaultValue;
  }

  mapOrElse<T>(defaultFn: () => T, _fn: (value: never) => T): T {
    return defaultFn();
  }

  okOr<E extends ToString>(error: E): Result<never, E> {
    return Err(error);
  }

  okOrElse<E extends ToString>(err: () => E): Result<never, E> {
    return Err(err());
  }

  and<T>(_other: Option<T>): None {
    return None;
  }

  andThen<T>(_fn: (value: never) => Option<T>): None {
    return None;
  }

  filter(_predicate: (value: never) => boolean): None {
    return None;
  }

  or<T>(other: Option<T>): Option<T> {
    return other;
  }

  orElse<T>(fn: () => Option<T>): Option<T> {
    return fn();
  }

  xor<T>(other: Option<T>): Option<T> {
    if (other.isSome()) {
      return other;
    } else {
      return None;
    }
  }

  contains(_value: never): false {
    return false;
  }

  zip<T>(_other: Option<T>): None {
    return None;
  }

  zipWith<T, K>(_other: Option<T>, _fn: (self: never, other: T) => K): None {
    return None;
  }

  unwrap(): never {
    return this.expect('called `Option::unwrap()` on a `None` value');
  }

  unwrapOr<T>(defaultValue: T): T {
    return defaultValue;
  }

  unwrapOrElse<T>(fn: () => T): T {
    return fn();
  }

  expect(message: string): never {
    throw new Error(message);
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

  isSome(): this is Some<S> {
    return true;
  }

  isSomeAnd(op: (value: Readonly<S>) => boolean): this is Some<S> {
    return op(this.value);
  }

  isNone(): this is None {
    return false;
  }

  match<T>(matcher: { some: (value: Readonly<S>) => T }): T {
    return matcher.some(this.value);
  }

  map<T>(fn: (value: Readonly<S>) => T): Some<T> {
    return Some(fn(this.value));
  }

  mapOr<T>(_defaultValue: T, fn: (value: Readonly<S>) => T): T {
    return fn(this.value);
  }

  mapOrElse<T>(_defaultFn: () => T, fn: (value: Readonly<S>) => T): T {
    return fn(this.value);
  }

  okOr<E extends ToString>(_error: E): Result<S, never> {
    return Ok(this.value);
  }

  okOrElse<E extends ToString>(_err: () => E): Result<S, never> {
    return Ok(this.value);
  }

  and<T>(other: Option<T>): typeof other {
    return other;
  }

  andThen<T>(fn: (value: Readonly<S>) => Option<T>): Option<T> {
    return fn(this.value);
  }

  filter(predicate: (value: Readonly<S>) => boolean): Option<S> {
    return predicate(this.value) ? this : None;
  }

  or<T>(_other: Option<T>): Some<S> {
    return this;
  }

  orElse<T>(_fn: () => Option<T>): Some<S> {
    return this;
  }

  xor<T>(other: Option<T>): Option<S> {
    if (other.isNone()) {
      return this;
    } else {
      return None;
    }
  }

  contains(value: S): boolean {
    return this.value === value;
  }

  zip<T>(other: Option<T>): Option<[S, T]> {
    return other.map((otherValue) => [this.value, otherValue]);
  }

  zipWith<T, K>(other: Option<T>, fn: (self: S, other: T) => K): Option<K> {
    return other.map((otherValue) => fn(this.value, otherValue));
  }

  unwrap(): S {
    return this.value;
  }

  unwrapOr<T>(_defaultValue: T): S {
    return this.value;
  }

  unwrapOrElse<T>(_fn: () => T): S {
    return this.value;
  }

  expect(_message: string): S {
    return this.value;
  }
}

export const Some = <T>(value: T) => new SomeImpl(value);
export type Some<S> = SomeImpl<S>;

export type Option<S> = Some<S> | None;
