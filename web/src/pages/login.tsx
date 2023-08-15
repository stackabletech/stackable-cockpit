import { JSX, Show, createResource, createSignal, untrack } from 'solid-js';
import { isLoggedIn, logIn, validateSessionOrLogOut } from '@/api/session';
import { translate } from '@/localization';

import { PasswordInput } from '@/components/form/password-input';
import { TextInput } from '@/components/form/text-input';
import { Button } from '@/components/button';

import loginImage from '@/resources/login.png';
import logo from '@/resources/logo.png';
import { LanguagePicker } from '@/components/language';
import { Wrapper } from '@/components/layout';

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
  const [loading, setLoading] = createSignal(false);

  const [currentAttempt, setCurrentAttempt] = createSignal(
    Promise.resolve<string | undefined>(undefined),
  );
  // Create attempts imperatively, use createResource to render the results
  const [loginMessage] = createResource(
    () => currentAttempt(),
    (attempt) => {
      // We do this to have a better UX
      setTimeout(() => setLoading(false), 300);
      return attempt;
    },
  );
  const clickLogin = () => {
    setLoading(true);
    void setCurrentAttempt(() => logIn(username(), password()));
  };
  return (
    <Wrapper>
      <div class='col-start-3 col-span-4 flex items-center h-screen'>
        <div class='w-full grid grid-cols-2'>
          <div class='bg-stackable-blue-light py-4 rounded-l-lg min-h-300px'>
            <img
              src={loginImage}
              alt='Login image'
              class='h-full w-full object-cover'
            />
          </div>
          <div class='bg-gray-800 p-4 flex flex-col justify-between rounded-r-lg'>
            <div class='flex justify-between items-center'>
              <img
                src={logo}
                elementtiming='logo'
                fetchpriority='auto'
                alt='Stackable Logo'
                class='h-20px'
              />
              <LanguagePicker />
            </div>
            <div>
              <h2 class='m-0 mb-4 c-white font-medium font-base text-xl leading-6'>
                {translate('login--log-in')}
              </h2>
              <form class='flex flex-col gap-4'>
                <TextInput
                  onInput={(event) => setUsername(event.target.value)}
                  placeholder={translate('login--username')}
                />
                <PasswordInput
                  onInput={(event) => setPassword(event.target.value)}
                />
                <div class='flex justify-end'>
                  {/* <Show when={loginMessage.loading}>
                  <div>logging in...</div>
                </Show>
                <Show when={loginMessage()}>
                  <div class='c-red'>{loginMessage()}</div>
                </Show> */}
                  <Button
                    text={translate('login--log-in')}
                    onClick={clickLogin}
                    loading={loading()}
                    role='primary'
                  />
                </div>
              </form>
            </div>
          </div>
        </div>
      </div>
    </Wrapper>
  );
};
