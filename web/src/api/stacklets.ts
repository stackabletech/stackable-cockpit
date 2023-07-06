import { components } from './schema';
import { client } from './session';
import { ObjectMeta } from './meta';
import { delay } from './mock-utils';

export type Stacklet = components['schemas']['Stacklet'];
export type DisplayCondition = components['schemas']['DisplayCondition'];
export async function getStacklets(): Promise<Stacklet[]> {
  const { data } = await client().get('/stacklets', {});
  if (data === undefined) {
    throw new Error('No data returned by API');
  } else {
    return data;
  }
}

export type DiscoveryFieldType = 'url' | 'blob';
interface StackletDiscovery {
  metadata: ObjectMeta;
  data: { [x: string]: string };
  fieldTypes: { [x: string]: DiscoveryFieldType };
}

export async function getStackletDiscovery(
  namespace: string,
  discoveryConfigMapName: string,
): Promise<StackletDiscovery | undefined> {
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
