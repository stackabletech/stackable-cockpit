import createClient from 'openapi-fetch';
import { components, paths } from './schema';

const client = createClient<paths>({ baseUrl: '/api' });

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

export type Stacklet = components['schemas']['Stacklet'];
export type DisplayCondition = components['schemas']['DisplayCondition'];
export async function getStacklets(): Promise<Stacklet[]> {
  const { data } = await client.get('/stacklets', {});
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
