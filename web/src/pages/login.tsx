import { JSX, Show, createResource, createSignal } from 'solid-js';
import { isLoggedIn, logIn } from '../api';

interface LoginPageOrProps {
  children: JSX.Element;
}
export const LoginPageOr = (props: LoginPageOrProps) => {
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
    // Doesn't typecheck without the undefined
    // eslint-disable-next-line unicorn/no-useless-undefined
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
            value={password()}
            onInput={(event) => setPassword(event.target.value)}
          />
        </label>
        <input type='submit' value='log in' onClick={clickLogin} />
      </form>
    </>
  );
};
