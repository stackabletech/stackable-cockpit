import { Show, createResource } from 'solid-js';
import { getProductClusters } from '../../api/index';
import { DataTable } from '../../components/datatable';
import { A } from '@solidjs/router';

export const ProductClusters = () => {
  const [productClusters, { refetch }] = createResource(getProductClusters);
  return (
    <>
      <button onClick={refetch}>Refresh</button>
      <Show when={productClusters.loading}>Loading...</Show>
      <DataTable
        items={productClusters() || []}
        columns={[
          {
            label: 'Product',
            get: (cluster) => cluster.product,
            sortBy: (cluster) => cluster.product,
          },
          {
            label: 'Namespace',
            get: (cluster) => cluster.metadata.namespace,
            sortBy: (cluster) => cluster.metadata.namespace,
          },
          {
            label: 'Name',
            get: (cluster) => cluster.metadata.name,
            sortBy: (cluster) => cluster.metadata.name,
          },
          {
            label: 'Actions',
            get: (cluster) => (
              <ul class='m-0 p-0'>
                <li class='inline-block mx-1 p-1 bg-gray'>
                  <A
                    class='color-black'
                    href={`/product-clusters/${cluster.metadata.namespace}/${cluster.metadata.name}/connect`}
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
