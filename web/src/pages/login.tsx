import {
  JSX,
  Show,
  createResource,
  createSignal,
  createUniqueId,
  untrack,
} from 'solid-js';
import logo from '../resources/logo.png';
import { isLoggedIn, logIn, validateSessionOrLogOut } from '../api/session';
import { Button } from '../components/button';
import { translate } from '../localization';

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
    Promise.resolve<string | undefined>(undefined),
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
    <div class='p-20 min-w-2xl m-auto pt-2'>
      <div class='rounded-xl px-7 pb-6 pt-9 bg-gray-800 m-auto flex flex-col flex-items-center'>
        <h1 class='m-0'>
          <img
            src={logo}
            elementtiming='logo'
            fetchpriority='auto'
            alt='Stackable'
          />
        </h1>
        <h2 class='mt-4 mb-3 text-xl font-normal'>
          {translate('login--log-in-needed')}
        </h2>
        <form
          class='grid gap-y-2'
          style={{
            'grid-template-columns': '[label] auto [field] 1fr',
            width: '100%',
          }}
        >
          <label class='me-2 text-lg' for={usernameId}>
            {translate('login--username')}
          </label>
          <input
            id={usernameId}
            value={username()}
            onInput={(event) => setUsername(event.target.value)}
          />
          <label class='me-2 text-lg' for={passwordId}>
            {translate('login--password')}
          </label>
          <input
            id={passwordId}
            type='password'
            value={password()}
            onInput={(event) => setPassword(event.target.value)}
          />
          <div
            class='flex flex-col flex-items-center mt-4'
            style={{ 'grid-column': 'span 2' }}
          >
            <Show when={loginMessage.loading}>
              <div>{translate('login--logging-in')}</div>
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
