import { JSX, Show, createResource, createSignal, untrack } from 'solid-js';
import { isLoggedIn, logIn, validateSessionOrLogOut } from '../api';

interface LoginPageOrProps {
  children: JSX.Element;
}
export const LoginPageOr = (props: LoginPageOrProps) => {
  // Validate the session when loading, but there's little to no point re-validating
  // when the user has just logged in.
  untrack(() => validateSessionOrLogOut());
  return (
    <>
      <Show when={isLoggedIn()}>{props.children}</Show>
      <Show when={!isLoggedIn()}>
        <LoginPage />
      </Show>
    </>
  );
};

export const LoginPage = () => {
  const [username, setUsername] = createSignal('');
  const [password, setPassword] = createSignal('');
  const [currentAttempt, setCurrentAttempt] = createSignal(
    Promise.resolve<string | undefined>(undefined),
  );
  // Create attempts imperatively, use createResource to render the results
  const [loginMessage] = createResource(
    () => currentAttempt(),
    (attempt) => attempt,
  );
  const clickLogin = (event: MouseEvent) => {
    event.preventDefault();
    void setCurrentAttempt(() => logIn(username(), password()));
  };
  return (
    <>
      <h2>login pl0x</h2>
      <form>
        <Show when={loginMessage.loading}>
          <div>logging in...</div>
        </Show>
        <Show when={loginMessage()}>
          <div class='c-red'>{loginMessage()}</div>
        </Show>
        <label>
          username
          <input
            value={username()}
            onInput={(event) => setUsername(event.target.value)}
          />
        </label>
        <label>
          password
          <input
            type='password'
            value={password()}
            onInput={(event) => setPassword(event.target.value)}
          />
        </label>
        <input type='submit' value='log in' onClick={clickLogin} />
      </form>
    </>
  );
};
