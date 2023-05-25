import { None, Option, Some } from './option';

export const Ok = <O, E = never>(value: O) => {
  return new Result<O, E>(false, value);
};

// eslint-disable-next-line unicorn/prevent-abbreviations
export const Err = <E, O = never>(error: E) => {
  return new Result<O, E>(true, error);
};

export class Result<O, E> {
  private is_error: boolean;
  private value: O | E;

  constructor(is_error: boolean, value: O | E) {
    this.is_error = is_error;
    this.value = value;
  }

  isOk(): this is Result<O, never> {
    return !this.is_error;
  }

  isOkAnd(op: (value: O) => boolean): this is Result<O, never> {
    return this.isOk() && op(this.value);
  }

  isErr(): this is Result<never, E> {
    return this.is_error;
  }

  isErrAnd(op: (error: E) => boolean): this is Result<never, E> {
    return this.isErr() && op(this.value);
  }

  match<T, K>(matcher: { ok: (value: O) => T; err: (error: E) => K }): T | K {
    return this.is_error
      ? matcher.err(this.value as E)
      : matcher.ok(this.value as O);
  }

  unwrap(): O {
    return this.match({
      ok: (value) => value,
      err: () => {
        throw new Error('tried to unwrap result value but found error');
      },
    });
  }

  unwrapOr(defaultValue: O): O {
    return this.match({
      ok: (value) => value,
      err: () => defaultValue,
    });
  }

  unwrapOrElse(op: (error: E) => O): O {
    return this.match({
      ok: (value) => value,
      err: (error) => op(error),
    });
  }

  ok(): Option<O> {
    return this.match({
      ok: (value) => Some(value),
      err: () => None(),
    });
  }
}
