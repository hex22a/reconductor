<script lang="ts">
  import CreateScanForm from '@/lib/components/scans/CreateScanForm.svelte';
  import type { PageProps } from './$types';
  import Table from '@/lib/components/Table.svelte';
  import { create } from '@/lib/services/scans';
  import { csrf } from '@/lib/stores/csrf';
  import { get } from 'svelte/store';
  import { isError } from '@/lib/transport/ErrorResponse';
  import { resolve } from '$app/paths';

  let { data }: PageProps = $props();
  let date = new Date(data.project!.created_at!).toUTCString();
  let scans = $state(data.scans.data || []);

  async function createScan(projectId: string, target: string, schedule: string) {
    const csrfToken = get(csrf);
    const scan = await create(csrfToken, projectId, target, schedule);

    if (isError(scan)) {
      return scan;
    }

    scans = [scan, ...scans];
  }
</script>

{#snippet linkCell(value: string | Date, id: string)}
  <a href={resolve(`/project/${data.project.id}/scan/${id}`)}>{value}</a>
{/snippet}

{#snippet dateCell(value: string | Date)}
  {new Date(value).toUTCString()}
{/snippet}

<div class="font-special">Project details</div>
<div>Name: {data.project.name}</div>
<div>Created At: {date}</div>
<CreateScanForm onCreate={createScan} />
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
  rows={scans}
/>
