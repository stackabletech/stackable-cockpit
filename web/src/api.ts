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

interface ProductCluster {
  metadata: ObjectMeta;
  product: string;
}

export async function getProductClusters(): Promise<ProductCluster[]> {
  await delay(200);
  return [
    {
      metadata: { namespace: 'default', name: 'simple-nifi' },
      product: 'nifi',
    },
  ];
}

interface ProductClusterDiscovery {
  metadata: ObjectMeta;
  data: { [x: string]: string };
}

export async function getProductClusterDiscovery(
  namespace: string,
  discoveryConfigMapName: string,
): Promise<ProductClusterDiscovery | undefined> {
  await delay(200);
  if (namespace == 'default' && discoveryConfigMapName == 'simple-nifi') {
    return {
      metadata: { namespace, name: discoveryConfigMapName },
      data: { url: 'https://foo.com' },
    };
  } else {
    return undefined;
  }
}
