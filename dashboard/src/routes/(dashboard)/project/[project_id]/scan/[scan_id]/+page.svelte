<script lang="ts">
  import Table from '@/lib/components/Table.svelte';
  import type { PageProps } from './$types';
  import { page } from '$app/state';
  let { data }: PageProps = $props();
  let date = new Date(data.scan_details.created_at!).toUTCString();
</script>

{#snippet dateCell(value: string | Date, id: string)}
  <a href="/project/{page.params.project_id}/scan/{data.scan_details.id}/run/{id}"
    >{new Date(value).toUTCString()}</a
  >
{/snippet}

<div class="font-special">Scan Details</div>
<div>Target: {data.scan_details.target}</div>
<div>Schedle: {data.scan_details.schedule}</div>
<div>Created At: {date}</div>
<h1 class="font-special">Runs</h1>
<Table
  columns={[
    {
      key: 'created_at',
      label: 'Created At',
      render: dateCell,
    },
  ]}
  rows={data.runs.data}
/>
