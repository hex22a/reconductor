<script lang="ts">
  import Table from '@/lib/components/Table.svelte';
  import type { PageProps } from './$types';
  import { page } from '$app/state';
  let { data }: PageProps = $props();
  let date = new Date(data.run_details.created_at!).toUTCString();
</script>

{#snippet linkCell(value: string | number | undefined, id: string)}
  <a
    href="/project/{page.params.project_id}/scan/{page.params.scan_id}/run/{data.run_details
      .id}/host/{id}">{value}</a
  >
{/snippet}

<div class="font-special">Scan Run Details</div>
<div>Created At: {date}</div>
<h1 class="font-special">Runs</h1>
<Table
  columns={[
    {
      key: 'ip',
      label: 'IP Address',
      render: linkCell,
    },
    { key: 'mac', label: 'Mac Address' },
    { key: 'hostname', label: 'Hostmane' },
    { key: 'vendor', label: 'Vendor' },
    { key: 'os_match', label: 'OS Match' },
    { key: 'os_accuracy', label: 'OS Accuracy' },
  ]}
  rows={data.hosts.data}
/>
