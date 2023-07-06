import { ObjectMeta } from './meta';
import { delay } from './mock-utils';

interface Listener {
  metadata: ObjectMeta;
  product: string;
  endpoints: ListenerEndpoint[];
}

interface ListenerEndpoint {
  path: string;
  web: boolean;
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
