import { Show, createResource } from 'solid-js';
import { getStacklets } from '../../api';
import { DataTable } from '../../components/datatable';
import { A } from '@solidjs/router';
import { Button, ButtonLink } from '../../components/button';
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
              <ButtonLink
                href={`/stacklets/${x.metadata.namespace}/${x.metadata.name}/connect`}
              >
                Connect
              </ButtonLink>
            ),
          },
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
