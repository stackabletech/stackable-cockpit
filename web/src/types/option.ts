export const Some = <S>(value: S) => {
  return new Option<S>(false, value);
};

export const None = () => {
  return new Option<never>(true);
};

export class Option<S> {
  private is_none: boolean;
  private value: S | never | undefined;

  constructor(is_none: boolean, value?: S) {
    this.is_none = is_none;
    this.value = value;
  }

  isSome(): this is Option<S> {
    return !this.is_none;
  }

  isSomeAnd(op: (value: S) => boolean): this is Option<S> {
    return this.isSome() && op(this.value as S);
  }

  isNone(): this is Option<undefined> {
    return this.is_none;
  }

  match<T, K>(matcher: { some: (value: S) => T; none: () => K }): T | K {
    // eslint-disable-next-line unicorn/no-array-callback-reference
    return this.is_none ? matcher.none() : matcher.some(this.value as S);
  }

  unwrap(): S {
    return this.match({
      some: (value) => value,
      none: () => {
        throw new Error('tried to unwrap option value but found none');
      },
    });
  }

  unwrapOr(defaultValue: S): S {
    return this.match({
      some: (value) => value,
      none: () => defaultValue,
    });
  }

  unwrapOrElse(op: () => S): S {
    return this.match({
      some: (value) => value,
      none: () => op(),
    });
  }
}
