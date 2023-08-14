import {
  JSX,
  Show,
  createResource,
  createSignal,
  createUniqueId,
  untrack,
} from 'solid-js';
import { isLoggedIn, logIn, validateSessionOrLogOut } from '../api/session';
import { Button } from '../components/button';
import { translate } from '../localization';

import logo from '../resources/logo.png';
import loginImage from '../resources/login.png';

interface LoginPageOrProps {
  children: JSX.Element;
}
export const LoginPageOr = (props: LoginPageOrProps) => {
  // Validate the session when loading, but there's little to no point re-validating
  // when the user has just logged in.
  untrack(() => validateSessionOrLogOut());
  return (
    <Show when={isLoggedIn()} fallback={<LoginPage />}>
      {props.children}
    </Show>
  );
};

export const LoginPage = () => {
  const [username, setUsername] = createSignal('');
  const [password, setPassword] = createSignal('');
  const [currentAttempt, setCurrentAttempt] = createSignal(
    Promise.resolve<string | undefined>(),
  );
  // Create attempts imperatively, use createResource to render the results
  const [loginMessage] = createResource(
    () => currentAttempt(),
    (attempt) => attempt,
  );
  const clickLogin = () => {
    void setCurrentAttempt(() => logIn(username(), password()));
  };
  const usernameId = createUniqueId();
  const passwordId = createUniqueId();
  return (
    <div class='p-16'>
      <div class='p-4 pt-0 max-w-2xl bg-gray-800 m-auto flex flex-col flex-items-center'>
        <h1 class='m-0'>
          <img
            src={logo}
            elementtiming='logo'
            fetchpriority='auto'
            alt='Stackable'
          />
        </h1>
        <h2 class='mt-0 mb-1'>{translate('login--log-in')}</h2>
        <form
          class='grid'
          style={{
            'grid-template-columns': '[label] auto [field] 1fr',
            width: '100%',
          }}
        >
          <label class='me-2' for={usernameId}>
            {translate('login--username')}
          </label>
          <input
            id={usernameId}
            value={username()}
            onInput={(event) => setUsername(event.target.value)}
          />
          <label class='me-2' for={passwordId}>
            {translate('login--password')}
          </label>
          <input
            id={passwordId}
            type='password'
            value={password()}
            onInput={(event) => setPassword(event.target.value)}
          />
          <div
            class='flex flex-col flex-items-center mt-2'
            style={{ 'grid-column': 'span 2' }}
          >
            <Show when={loginMessage.loading}>
              <div>logging in...</div>
            </Show>
            <Show when={loginMessage()}>
              <div class='c-red'>{loginMessage()}</div>
            </Show>
            <Button onClick={clickLogin} role='primary'>
              {translate('login--log-in')}
            </Button>
          </div>
        </form>
      </div>
    </div>
  );
};
