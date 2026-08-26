<script lang="ts">
  import Table from '@/lib/components/Table.svelte';
  import type { PageProps } from './$types';
  let { data }: PageProps = $props();
</script>

<div class="font-special">Host Details</div>
<div>Host: {data.host_details.hostname}</div>
<div>IP: {data.host_details.ip}</div>
<h1 class="font-special">Ports</h1>
<Table
  columns={[
    { key: 'port', label: 'Port #' },
    { key: 'protocol', label: 'Protocol' },
    { key: 'state', label: 'State' },
    { key: 'service', label: 'Service' },
    { key: 'product', label: 'Product' },
    { key: 'version', label: 'Version' },
  ]}
  rows={data.ports.data}
/>
