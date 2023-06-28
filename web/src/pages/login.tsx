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
    Promise.resolve<string | undefined>(undefined),
  );
  // Create attempts imperatively, use createResource to render the results
  const [loginMessage] = createResource(
    () => currentAttempt(),
    (attempt) => attempt,
  );
  const clickLogin = (e: MouseEvent) => {
    e.preventDefault();
    setCurrentAttempt(() => logIn(username(), password()));
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
            onInput={(e) => setUsername(e.target.value)}
          />
        </label>
        <label>
          password
          <input
            value={password()}
            onInput={(e) => setPassword(e.target.value)}
          />
        </label>
        <input type='submit' value='log in' onClick={clickLogin} />
      </form>
    </>
  );
};
