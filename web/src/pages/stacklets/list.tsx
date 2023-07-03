import { createResource } from 'solid-js';
import { getStacklets } from '../../api';
import { DataTable } from '../../components/datatable';
import { ButtonLink } from '../../components/button';
import { AddSymbol } from '../../components/symbols';

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
            get: (x) => x.namespace || '(Cluster-scoped)',
            sortBy: (x) => x.namespace || '',
          },
          {
            label: 'Name',
            get: (x) => x.name,
            sortBy: (x) => x.name,
          },
          /* {
            label: 'Actions',
            get: (x) => (
              <ButtonLink href={`/stacklets/${x.namespace}/${x.name}/connect`}>
                Connect
              </ButtonLink>
            ),
          }, */
        ]}
        extraButtons={
          <ButtonLink href='/stacklets/add' role='primary'>
            <AddSymbol /> Add stacklet
          </ButtonLink>
        }
        refresh={refetch}
        isLoading={stacklets.loading}
      />
    </>
  );
};
