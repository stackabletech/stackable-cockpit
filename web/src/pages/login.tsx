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
    <div class='col-start-3 col-span-4 flex items-center h-screen'>
      <div class='w-full grid grid-cols-2'>
        <div class='bg-stackable-blue-light p-4 rounded-l-lg min-h-300px'>
          <img
            src={loginImage}
            alt='Login image'
            class='h-full w-full object-cover'
          />
        </div>
        <div class='bg-gray-800 p-4 flex flex-col justify-between rounded-r-lg'>
          <div>
            <img
              src={logo}
              elementtiming='logo'
              fetchpriority='auto'
              alt='Stackable Logo'
              class='h-20px'
            />
          </div>
          <div>
            <h2 class='m-0 mb-4 c-white font-medium font-base text-xl leading-6'>
              Login
            </h2>
            <form
              class='grid'
              style={{
                'grid-template-columns': '[label] auto [field] 1fr',
                width: '100%',
              }}
            >
              <label class='me-2' for={usernameId}>
                Username
              </label>
              <input
                id={usernameId}
                value={username()}
                onInput={(event) => setUsername(event.target.value)}
              />
              <label class='me-2' for={passwordId}>
                Password
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
                  Log in
                </Button>
              </div>
            </form>
          </div>
        </div>
      </div>
    </div>
  );
};
