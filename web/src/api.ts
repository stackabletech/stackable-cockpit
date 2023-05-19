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
    {
      metadata: { namespace: 'default', name: 'simple-hdfs' },
      product: 'hdfs',
    },
  ];
}

export type DiscoveryFieldType = 'url' | 'blob';
interface ProductClusterDiscovery {
  metadata: ObjectMeta;
  data: { [x: string]: string };
  fieldTypes: { [x: string]: DiscoveryFieldType };
}

export async function getProductClusterDiscovery(
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
