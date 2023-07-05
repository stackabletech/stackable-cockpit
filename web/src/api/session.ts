import openapiCreateClient from 'openapi-fetch';
import { paths } from './schema';
import { createLocalStorageSignal } from '../utils/localstorage';
import { None, someIfDefined } from '../types';
import { createMemo } from 'solid-js';

const createClient = (options: RequestInit) =>
  openapiCreateClient<paths>({ baseUrl: '/api', ...options });
const [currentSessionToken, setCurrentSessionToken] =
  createLocalStorageSignal('sessionToken');
export const isLoggedIn = () => currentSessionToken().isSome();
export const client = createMemo(() => {
  const headers: HeadersInit = {};
  currentSessionToken().map((sessionToken) => {
    headers.Authorization = `Bearer ${sessionToken}`;
  });
  return createClient({ headers });
});

// Try to validate that the session token is still valid, and log the user out otherwise
export function validateSessionOrLogOut() {
  if (isLoggedIn()) {
    client()
      .get('/ping', {})
      .then((pingResponse) => {
        if (pingResponse.response.status === 401) {
          setCurrentSessionToken(None);
        }
      })
      // We don't want to block page loads on waiting for this validation
      // eslint-disable-next-line unicorn/prefer-top-level-await
      .catch((error) =>
        console.error('Failed to validate session token', error),
      );
  }
}

export async function logIn(
  username: string,
  password: string,
): Promise<string | undefined> {
  // Always use unauthenticated client for login requests
  const response = await createClient({}).post('/login', {
    headers: { Authorization: 'Basic ' + btoa(`${username}:${password}`) },
  });
  setCurrentSessionToken(someIfDefined(response.data?.sessionToken));
  if (!response.response.ok) {
    return response.error;
  }
}
// We want to leave room in the function contract to invalidate the session token in the future
// eslint-disable-next-line @typescript-eslint/require-await
export async function logOut() {
  setCurrentSessionToken(None);
}
