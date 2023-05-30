/* eslint-disable unicorn/prevent-abbreviations */
import { None, Option, Some } from './option';
import { ToString } from './utils';

export const Ok = <O extends ToString, E extends ToString = never>(
  value: O,
) => {
  return new Result<O, E>(false, value);
};

// eslint-disable-next-line unicorn/prevent-abbreviations
export const Err = <E extends ToString, O extends ToString = never>(
  error: E,
) => {
  return new Result<O, E>(true, error);
};

export class Result<O extends ToString, E extends ToString> {
  private is_error: boolean;
  private value: O | E;

  constructor(is_error: boolean, value: O | E) {
    this.is_error = is_error;
    this.value = value;
  }

  isOk(): this is Result<O, never> {
    return !this.is_error;
  }

  isOkAnd(fn: (value: Readonly<O>) => boolean): this is Result<O, never> {
    return this.isOk() && fn(this.value);
  }

  isErr(): this is Result<never, E> {
    return this.is_error;
  }

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

  unwrap(): O {
    return this.expect('called `Result.unwrap()` on an `Err` value');
  }

  unwrapErr(): E {
    return this.expectErr('called Result.unwrapErr() on an `Ok` value');
  }

  unwrapOr(defaultValue: O): O {
    return this.match({
      ok: (value) => value,
      err: () => defaultValue,
    });
  }

  unwrapOrElse(fn: (error: Readonly<E>) => O): O {
    return this.match({
      ok: (value) => value,
      err: (error) => fn(error),
    });
  }

  ok(): Option<O> {
    return this.match({
      ok: (value) => Some(value),
      err: () => None(),
    });
  }

  err(): Option<E> {
    return this.match({
      ok: None,
      err: (error) => Some(error),
    });
  }

  map<T extends ToString>(fn: (value: Readonly<O>) => T): Result<T, E> {
    return this.match({
      ok: (value) => Ok(fn(value)),
      err: (error) => Err(error),
    });
  }

  mapOr<T>(defaultValue: T, fn: (value: Readonly<O>) => T): T {
    return this.match({
      ok: (value) => fn(value),
      err: () => defaultValue,
    });
  }

  mapOrElse<T>(
    defaultFn: (error: Readonly<E>) => T,
    fn: (value: Readonly<O>) => T,
  ): T {
    return this.match({
      ok: (value) => fn(value),
      err: (error) => defaultFn(error),
    });
  }

  mapErr<T extends string>(fn: (error: Readonly<E>) => T): Result<O, T> {
    return this.match({
      ok: (value) => Ok(value),
      err: (error) => Err(fn(error)),
    });
  }

  expect(msg: string): O {
    return this.match({
      ok: (value) => value,
      err: (error) => {
        throw new Error(`${msg}: ${error.toString()}`);
      },
    });
  }

  expectErr(msg: string): E {
    return this.match({
      ok: (value) => {
        throw new Error(`${msg}: ${value.toString()}`);
      },
      err: (error) => error,
    });
  }
}
