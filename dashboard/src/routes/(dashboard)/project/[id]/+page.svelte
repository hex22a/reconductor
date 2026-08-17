<script lang="ts">
  import CreateScanForm from '@/lib/components/scans/CreateScanForm.svelte';
  import type { PageProps } from './$types';
  import Table from '@/lib/components/Table.svelte';
  import { scansStore } from '@/lib/components/scans/scans.svelte';
  let { data }: PageProps = $props();
  let date = new Date(data.created_at!).toUTCString();
</script>

{#snippet linkCell(value: string | Date, id: string)}
  <a href="/scan/{id}">{value}</a>
{/snippet}

{#snippet dateCell(value: string | Date)}
  {new Date(value).toUTCString()}
{/snippet}

<div class="font-special">Project details</div>
<div>Name: {data.name}</div>
<div>Created At: {date}</div>
<CreateScanForm />
<h1 class="font-special">Scans</h1>
<Table
  columns={[
    {
      key: 'target',
      label: 'Target',
      render: linkCell,
    },
    {
      key: 'schedule',
      label: 'Schedule',
    },
    {
      key: 'created_at',
      label: 'Created At',
      render: dateCell,
    },
  ]}
  rows={scansStore.scans}
/>
