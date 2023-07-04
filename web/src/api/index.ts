import openapiCreateClient from 'openapi-fetch';
import { components, paths } from './schema';
import { createLocalStorageSignal } from '../utils/localstorage';
import { None, someIfDefined } from '../types';
import { createMemo } from 'solid-js';

const createClient = (options: RequestInit) =>
  openapiCreateClient<paths>({ baseUrl: '/api', ...options });
const [currentSessionToken, setCurrentSessionToken] =
  createLocalStorageSignal('sessionToken');
export const isLoggedIn = () => currentSessionToken().isSome();
const client = createMemo(() => {
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

interface ObjectMeta {
  namespace: string;
  name: string;
}

interface Listener {
  metadata: ObjectMeta;
  product: string;
  endpoints: ListenerEndpoint[];
}

interface ListenerEndpoint {
  path: string;
  web: boolean;
}

function delay(amount: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, amount));
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

export async function getListeners(): Promise<Listener[]> {
  await delay(200);
  return [
    {
      metadata: {
        namespace: 'default',
        name: 'simple-nifi',
      },
      product: 'nifi',
      endpoints: [
        {
          path: 'https://127.0.0.1:8443/ui',
          web: true,
        },
        {
          path: 'mqtt://127.0.0.1:9999',
          web: false,
        },
      ],
    },
    {
      metadata: {
        namespace: 'default',
        name: 'dimple-nifi',
      },
      product: 'nifi',
      endpoints: [
        {
          path: 'https://127.0.0.1:8443/ui',
          web: true,
        },
        {
          path: 'mqtt://127.0.0.1:9999',
          web: false,
        },
      ],
    },
    {
      metadata: {
        namespace: 'sefault',
        name: 'dimple-nifi',
      },
      product: 'nifi',
      endpoints: [
        {
          path: 'https://127.0.0.1:8443/ui',
          web: true,
        },
        {
          path: 'mqtt://127.0.0.1:9999',
          web: false,
        },
      ],
    },
  ];
}

type Stacklet = components['schemas']['Stacklet'];
export async function getStacklets(): Promise<Stacklet[]> {
  const { data } = await client().get('/stacklets', {});
  if (data === undefined) {
    throw new Error('No data returned by API');
  } else {
    return data;
  }
}

export type DiscoveryFieldType = 'url' | 'blob';
interface ProductClusterDiscovery {
  metadata: ObjectMeta;
  data: { [x: string]: string };
  fieldTypes: { [x: string]: DiscoveryFieldType };
}

export async function getStackletDiscovery(
  namespace: string,
  discoveryConfigMapName: string,
): Promise<ProductClusterDiscovery | undefined> {
  await delay(200);
  if (namespace == 'default' && discoveryConfigMapName == 'simple-nifi') {
    return {
      metadata: { namespace, name: discoveryConfigMapName },
      data: { NIFI_URL: 'https://foo.com' },
      fieldTypes: { NIFI_URL: 'url' },
    };
  } else if (
    namespace == 'default' &&
    discoveryConfigMapName == 'simple-hdfs'
  ) {
    return {
      metadata: { namespace, name: discoveryConfigMapName },
      data: { 'hdfs-config.xml': '<?xml>config goes here' },
      fieldTypes: {},
    };
  } else {
    return undefined;
  }
}
