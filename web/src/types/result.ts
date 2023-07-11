import { None, Option, Some } from './option';
import { ToString } from './utils';

interface BaseResult<O, E extends ToString> {
  /**
   * Returns true if the result is `Ok`.
   *
   * @returns boolean
   */
  isOk(): this is Ok<O>;

  /**
   * Returns `true` if the result is `Ok` and the value inside of it matches
   * a predicate.
   *
   * @param fn Predicate to match.
   * @returns boolean
   */
  isOkAnd(fn: (value: Readonly<O>) => boolean): this is Ok<O>;

  /**
   * Returns `true` if the result is `Err`.
   *
   * @returns boolean
   */
  isErr(): this is Err<E>;

  /**
   * Returns `true` if the result is `Err` and the value inside of it matches
   * a predicate.
   *
   * @param fn Predicate to match.
   * @returns boolean
   */
  isErrAnd(fn: (error: Readonly<E>) => boolean): this is Err<E>;

  match<T, K>(matcher: {
    ok: (value: Readonly<O>) => T;
    err: (error: Readonly<E>) => K;
  }): T | K;

  /**
   * Returns the contained `Ok` value.
   *
   * Because this function may throw, its use is generally discouraged. Instead,
   * prefer to use pattern matching and handle the `Err` case explicitly, or
   * call `unwrap_or` or `unwrap_or_else`.
   *
   * @returns The contained `Ok` value.
   *
   * @throws Throws if the value is an `Err`, with a panic message provided by
   * the `Err`'s value.
   */
  unwrap(): O;

  /**
   * Returns the contained `Err` value.
   *
   * @returns The contained `Err` value.
   *
   * @throws Throws if the value is an `Ok`, with a custom panic message
   * provided by the `Ok`'s value.
   */
  unwrapErr(): E;

  /**
   * Returns the contained `Ok` value or a provided default.
   *
   * @param defaultValue The default value if `Err`.
   * @returns Either contained `Ok` value of default value.
   */
  unwrapOr(defaultValue: O): O;

  /**
   * Returns the contained `Ok` value or computes it from a closure.
   *
   * @param fn Computed the default value if `Err`.
   * @returns The contained `Ok` value or computed default value.
   */
  unwrapOrElse(fn: (error: Readonly<E>) => O): O;

  /**
   * Converts from `Result<O, E>` to `Option<O>`. Converts self into an
   * `Option<O>` and discarding the error, if any.
   *
   * @returns The converted option.
   */
  ok(): Option<O>;

  /**
   * Converts from `Result<O, E>` to `Option<E>`. Converts self into an
   * `Option<E>` and discarding the success value, if any.
   *
   * @returns The converted option.
   */
  err(): Option<E>;

  /**
   * Maps a `Result<O, E>` to `Result<T, E>` by applying a function to a
   * contained `Ok` value, leaving an `Err` value untouched.
   *
   * This function can be used to compose the results of two functions.
   *
   * @param fn The mapping function.
   * @returns The mapped result.
   */
  map<T extends ToString>(fn: (value: Readonly<O>) => T): Result<T, E>;

  /**
   * Returns the provided default (if `Err`), or applies a function to the
   * contained value (if `Ok`).
   *
   * @param defaultValue The default value of `Err`.
   * @param fn The mapping function if `Ok`.
   * @returns The mapped result.
   */
  mapOr<T>(defaultValue: T, fn: (value: Readonly<O>) => T): T;

  /**
   * Maps a `Result<O, E>` to `T` by applying fallback function default to a
   * contained `Err` value, or function `fn` to a contained `Ok` value.
   *
   * This function can be used to unpack a successful result while handling
   * an error.
   *
   * @param defaultFn Function computing the default value if `Err`.
   * @param fn Mapping function if `Ok`.
   * @returns The mapped value.
   */
  mapOrElse<T>(
    defaultFn: (error: Readonly<E>) => T,
    fn: (value: Readonly<O>) => T,
  ): T;

  /**
   * Maps a `Result<O, E>` to `Result<O, F>` by applying a function to a
   * contained `Err` value, leaving an `Ok` value untouched.
   *
   * This function can be used to pass through a successful result while
   * handling an error.
   *
   * @param fn Error mapping function.
   * @returns The mapped result.
   */
  mapErr<T extends ToString>(fn: (error: Readonly<E>) => T): Result<O, T>;

  /**
   * Returns the contained `Ok` value.
   *
   * Because this function may throw, its use is generally discouraged. Instead,
   * prefer to use pattern matching and handle the `Err` case explicitly, or
   * call `unwrap_or` or `unwrap_or_else`.
   *
   * @param message Error message.
   * @returns The contained `Ok` value.
   *
   * @throws Throws if the value is an `Err`, with a panic message including the
   * passed message, and the content of the `Err`.
   */
  expect(message: string): O;

  /**
   * Returns the contained `Err` value.
   *
   * @param message Error message.
   * @returns The contained `Err` value.
   *
   * @throws Throws if the value is an `Ok`, with an error message including the
   * passed message, and the content of the `Ok`.
   */
  expectErr(message: string): E;
}

export class OkImpl<O> implements BaseResult<O, never> {
  constructor(public value: O) {}

  isOk(): this is Ok<O> {
    return true;
  }

  isOkAnd(fn: (value: Readonly<O>) => boolean): this is Ok<O> {
    return fn(this.value);
  }

  isErr(): this is Err<never> {
    return false;
  }

  isErrAnd(_fn: (error: Readonly<never>) => boolean): this is Err<never> {
    return false;
  }

  match<T>(matcher: { ok: (value: Readonly<O>) => T }): T {
    return matcher.ok(this.value);
  }

  unwrap(): O {
    return this.value;
  }

  unwrapErr(): never {
    return this.expectErr('called `Result.unwrapErr()` on an `Ok` value');
  }

  unwrapOr(_defaultValue: O): O {
    return this.value;
  }

  unwrapOrElse(_fn: (error: Readonly<never>) => O): O {
    return this.value;
  }

  ok(): Option<O> {
    return Some(this.value);
  }

  err(): None {
    return None;
  }

  map<T extends ToString>(fn: (value: Readonly<O>) => T): Result<T, never> {
    return Ok(fn(this.value));
  }

  mapOr<T>(_defaultValue: T, fn: (value: Readonly<O>) => T): T {
    return fn(this.value);
  }

  mapOrElse<T>(
    _defaultFn: (error: Readonly<never>) => T,
    fn: (value: Readonly<O>) => T,
  ): T {
    return fn(this.value);
  }

  mapErr<T extends ToString>(_fn: (error: Readonly<never>) => T): Result<O, T> {
    return Ok(this.value);
  }

  expect(_message: string): O {
    return this.value;
  }

  expectErr(message: string): never {
    throw new Error(`${message}`);
  }
}

export const Ok = <O>(value: O) => new OkImpl(value);
export type Ok<O> = OkImpl<O>;

class ErrImpl<E extends ToString> implements BaseResult<never, E> {
  constructor(public error: E) {}

  isOk(): this is Ok<never> {
    return false;
  }

  isOkAnd(_fn: (value: never) => boolean): this is Ok<never> {
    return false;
  }

  isErr(): this is Err<E> {
    return true;
  }

  isErrAnd(fn: (error: Readonly<E>) => boolean): this is Err<E> {
    return fn(this.error);
  }

  match<T, K>(matcher: { err: (error: Readonly<E>) => K }): T | K {
    return matcher.err(this.error);
  }

  unwrap(): never {
    return this.expect('called `Result.unwrap()` on an `Err` value');
  }

  unwrapErr(): E {
    return this.error;
  }

  unwrapOr<T>(defaultValue: T): T {
    return defaultValue;
  }

  unwrapOrElse<T>(fn: (error: Readonly<E>) => T): T {
    return fn(this.error);
  }

  ok(): Option<never> {
    return None;
  }

  err(): Option<E> {
    return Some(this.error);
  }

  map<T extends ToString>(_fn: (value: never) => T): Result<never, E> {
    return Err(this.error);
  }

  mapOr<T>(defaultValue: T, _fn: (value: never) => T): T {
    return defaultValue;
  }

  mapOrElse<T>(
    defaultFn: (error: Readonly<E>) => T,
    _fn: (value: never) => T,
  ): T {
    return defaultFn(this.error);
  }

  mapErr<T extends ToString>(fn: (error: Readonly<E>) => T): Result<never, T> {
    return Err(fn(this.error));
  }

  expect(message: string): never {
    throw new Error(`${message}`);
  }

  expectErr(_message: string): E {
    return this.error;
  }
}

export const Err = <E extends ToString>(error: E) => new ErrImpl(error);
export type Err<E extends ToString> = ErrImpl<E>;

export type Result<O, E extends ToString> = Ok<O> | Err<E>;
