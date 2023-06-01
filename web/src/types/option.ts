/* eslint-disable unicorn/prevent-abbreviations */
import { ToString } from './utils';
import { Err, Ok, Result } from './result';

export const Some = <S extends ToString>(value: S) => {
  return new Option<S>(false, value);
};

export const None = () => {
  return new Option<never>(true);
};

export class Option<S extends ToString> {
  private is_none: boolean;
  private value: S | never | undefined;

  constructor(is_none: boolean, value?: S) {
    this.is_none = is_none;
    this.value = value;
  }

  isSome(): this is Option<S> {
    return !this.is_none;
  }

  isSomeAnd(op: (value: Readonly<S>) => boolean): this is Option<S> {
    return this.isSome() && op(this.value as S);
  }

  isNone(): this is Option<never> {
    return this.is_none;
  }

  match<T, K>(matcher: {
    some: (value: Readonly<S>) => T;
    none: () => K;
  }): T | K {
    // eslint-disable-next-line unicorn/no-array-callback-reference
    return this.is_none ? matcher.none() : matcher.some(this.value as S);
  }

  map<T extends ToString>(fn: (value: Readonly<S>) => T): Option<T> {
    return this.match({
      some: (value) => Some(fn(value)),
      none: None,
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

  and<T extends ToString>(other: Option<T>): Option<T> {
    return this.match({
      some: () => other,
      none: None,
    });
  }

  andThen<T extends ToString>(
    fn: (value: Readonly<S>) => Option<T>,
  ): Option<T> {
    return this.match({
      some: (value) => fn(value),
      none: None,
    });
  }

  filter(predicate: (value: Readonly<S>) => boolean): Option<S> {
    return this.match({
      some: (value) => (predicate(value) ? Some(value) : None()),
      none: None,
    });
  }

  or<T extends ToString>(other: Option<S | T>): Option<S | T> {
    return this.match({
      some: (value) => Some(value),
      none: () => other,
    });
  }

  orElse<T extends ToString>(fn: () => Option<S | T>): Option<S | T> {
    return this.match({
      some: (value) => Some(value),
      none: fn,
    });
  }

  xor<T extends ToString>(other: Option<S | T>): Option<S | T> {
    if (this.isSome() && other.isNone()) {
      return this;
    } else if (this.isNone() && other.isSome()) {
      return other;
    } else {
      return None();
    }
  }

  take(): Option<S> {
    return this.match({
      some: (value) => {
        this.value = undefined;
        this.is_none = true;

        return Some(value);
      },
      none: None,
    });
  }

  replace(value: S): Option<S> {
    return this.match({
      some: (oldValue) => {
        this.value = value;
        return Some(oldValue);
      },
      none: () => {
        this.value = value;
        this.is_none = false;

        return None();
      },
    });
  }

  contains(value: S): boolean {
    return this.match({
      some: (inner) => inner === value,
      none: () => false,
    });
  }

  zip<T extends ToString>(other: Option<T>): Option<[S, T]> {
    return this.isSome() && other.isSome()
      ? Some([this.unwrap(), other.unwrap()])
      : None();
  }

  zipWith<T extends ToString, K extends ToString>(
    other: Option<T>,
    fn: (self: S, other: T) => K,
  ): Option<K> {
    return this.isSome() && other.isSome()
      ? Some(fn(this.unwrap(), other.unwrap()))
      : None();
  }

  unwrap(): S {
    return this.expect('called `Option::unwrap()` on a `None` value');
  }

  unwrapOr<T extends ToString>(defaultValue: S | T): S | T {
    return this.match({
      some: (value) => value,
      none: () => defaultValue,
    });
  }

  unwrapOrElse<T extends ToString>(fn: () => S | T): S | T {
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
