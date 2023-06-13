import { Show, createResource } from 'solid-js';
import { getStacklets } from '../../api';
import { DataTable } from '../../components/datatable';
import { A } from '@solidjs/router';

export const Stacklets = () => {
  const [stacklets, { refetch }] = createResource(getStacklets);
  return (
    <>
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
            get: (x) => x.metadata.name,
            sortBy: (x) => x.metadata.name,
          },
          {
            label: 'Actions',
            get: (x) => (
              <ul class='m-0 p-0'>
                <li class='inline-block mx-1 p-1 bg-gray'>
                  <A
                    class='color-black'
                    href={`/stacklets/${x.metadata.namespace}/${x.metadata.name}/connect`}
                  >
                    Connect
                  </A>
                </li>
              </ul>
            ),
          },
        ]}
        refresh={refetch}
        isLoading={stacklets.loading}
      />
    </>
  );
};
