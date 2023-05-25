import { Show, createResource } from 'solid-js';
import { getStacklets } from '../../api/index';
import { DataTable } from '../../components/datatable';
import { A } from '@solidjs/router';

export const Stacklets = () => {
  const [stacklets, { refetch }] = createResource(getStacklets);
  return (
    <>
      <button onClick={refetch}>Refresh</button>
      <Show when={stacklets.loading}>Loading...</Show>
      <DataTable
        items={stacklets() || []}
        columns={[
          {
            label: 'Product',
            get: (x) => x.product,
            sortBy: (x) => x.product,
          },
          {
            label: 'Namespace',
            get: (x) => x.metadata.namespace,
            sortBy: (x) => x.metadata.namespace,
          },
          {
            label: 'Name',
            get: (cluster) => cluster.metadata.name,
            sortBy: (cluster) => cluster.metadata.name,
          },
          {
            label: 'Actions',
            get: (x) => (
              <ul class='m-0 p-0'>
                <li class='inline-block mx-1 p-1 bg-gray'>
                  <A
                    class='color-black'
                    href={`/product-clusters/${x.metadata.namespace}/${x.metadata.name}/connect`}
                  >
                    Connect
                  </A>
                </li>
              </ul>
            ),
          },
        ]}
      />
    </>
  );
};
