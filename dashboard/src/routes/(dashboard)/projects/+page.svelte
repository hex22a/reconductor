<script lang="ts">
  import CrateProjectForm from '@/lib/components/projects/CrateProjectForm.svelte';
  import { projectsStore } from '@/lib/components/projects/projects.svelte';
  import Table from '@/lib/components/Table.svelte';
</script>

{#snippet linkCell(value: string | Date, id: string)}
  <a href="/project/{id}">{value}</a>
{/snippet}

{#snippet dateCell(value: string | Date)}
  {new Date(value).toUTCString()}
{/snippet}

<CrateProjectForm />
<h1 class="font-special">Projects</h1>
<Table
  columns={[
    {
      key: 'name',
      label: 'Name',
      render: linkCell,
    },
    {
      key: 'created_at',
      label: 'Created At',
      render: dateCell,
    },
  ]}
  rows={projectsStore.projects}
/>
