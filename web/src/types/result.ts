import { None, Option, Some } from './option';
import { ToString } from './utils';

export const Ok = <O, E extends ToString = never>(value: O) => {
  return new Result<O, E>(false, value);
};

export const Err = <E extends ToString, O = never>(error: E) => {
  return new Result<O, E>(true, error);
};

export class Result<O, E extends ToString> {
  private is_error: boolean;
  private value: O | E;

  constructor(is_error: boolean, value: O | E) {
    this.is_error = is_error;
    this.value = value;
  }

  /**
   * Returns true if the result is `Ok`.
   *
   * @returns boolean
   */
  isOk(): this is Result<O, never> {
    return !this.is_error;
  }

  /**
   * Returns `true` if the result is `Ok` and the value inside of it matches
   * a predicate.
   *
   * @param fn Predicate to match.
   * @returns boolean
   */
  isOkAnd(fn: (value: Readonly<O>) => boolean): this is Result<O, never> {
    return this.isOk() && fn(this.value);
  }

  /**
   * Returns `true` if the result is `Err`.
   *
   * @returns boolean
   */
  isErr(): this is Result<never, E> {
    return this.is_error;
  }

  /**
   * Returns `true` if the result is `Err` and the value inside of it matches
   * a predicate.
   *
   * @param fn Predicate to match.
   * @returns boolean
   */
  isErrAnd(fn: (error: Readonly<E>) => boolean): this is Result<never, E> {
    return this.isErr() && fn(this.value);
  }

  match<T, K>(matcher: {
    ok: (value: Readonly<O>) => T;
    err: (error: Readonly<E>) => K;
  }): T | K {
    return this.is_error
      ? matcher.err(this.value as E)
      : matcher.ok(this.value as O);
  }

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
  unwrap(): O {
    return this.expect('called `Result.unwrap()` on an `Err` value');
  }

  /**
   * Returns the contained `Err` value.
   *
   * @returns The contained `Err` value.
   *
   * @throws Throws if the value is an `Ok`, with a custom panic message
   * provided by the `Ok`'s value.
   */
  unwrapErr(): E {
    return this.expectErr('called `Result.unwrapErr()` on an `Ok` value');
  }

  /**
   * Returns the contained `Ok` value or a provided default.
   *
   * @param defaultValue The default value if `Err`.
   * @returns Either contained `Ok` value of default value.
   */
  unwrapOr(defaultValue: O): O {
    return this.match({
      ok: (value) => value,
      err: () => defaultValue,
    });
  }

  /**
   * Returns the contained `Ok` value or computes it from a closure.
   *
   * @param fn Computed the default value if `Err`.
   * @returns The contained `Ok` value or computed default value.
   */
  unwrapOrElse(fn: (error: Readonly<E>) => O): O {
    return this.match({
      ok: (value) => value,
      err: (error) => fn(error),
    });
  }

  /**
   * Converts from `Result<O, E>` to `Option<O>`. Converts self into an
   * `Option<O>` and discarding the error, if any.
   *
   * @returns The converted option.
   */
  ok(): Option<O> {
    return this.match({
      ok: (value) => Some(value),
      err: () => None,
    });
  }

  /**
   * Converts from `Result<O, E>` to `Option<E>`. Converts self into an
   * `Option<E>` and discarding the success value, if any.
   *
   * @returns The converted option.
   */
  err(): Option<E> {
    return this.match({
      ok: () => None,
      err: (error) => Some(error),
    });
  }

  /**
   * Maps a `Result<O, E>` to `Result<T, E>` by applying a function to a
   * contained `Ok` value, leaving an `Err` value untouched.
   *
   * This function can be used to compose the results of two functions.
   *
   * @param fn The mapping function.
   * @returns The mapped result.
   */
  map<T extends ToString>(fn: (value: Readonly<O>) => T): Result<T, E> {
    return this.match({
      ok: (value) => Ok(fn(value)),
      err: (error) => Err(error),
    });
  }

  /**
   * Returns the provided default (if `Err`), or applies a function to the
   * contained value (if `Ok`).
   *
   * @param defaultValue The default value of `Err`.
   * @param fn The mapping function if `Ok`.
   * @returns The mapped result.
   */
  mapOr<T>(defaultValue: T, fn: (value: Readonly<O>) => T): T {
    return this.match({
      ok: (value) => fn(value),
      err: () => defaultValue,
    });
  }

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
  ): T {
    return this.match({
      ok: (value) => fn(value),
      err: (error) => defaultFn(error),
    });
  }

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
  mapErr<T extends ToString>(fn: (error: Readonly<E>) => T): Result<O, T> {
    return this.match({
      ok: (value) => Ok(value),
      err: (error) => Err(fn(error)),
    });
  }

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
  expect(message: string): O {
    return this.match({
      ok: (value) => value,
      err: (error) => {
        throw new Error(`${message}: ${error.toString()}`);
      },
    });
  }

  /**
   * Returns the contained `Err` value.
   *
   * @param message Error message.
   * @returns The contained `Err` value.
   *
   * @throws Throws if the value is an `Ok`, with an error message including the
   * passed message, and the content of the `Ok`.
   */
  expectErr(message: string): E {
    return this.match({
      ok: () => {
        throw new Error(`${message}`);
      },
      err: (error) => error,
    });
  }
}
