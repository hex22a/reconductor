<script lang="ts">
  import { csrf } from '#src/lib/stores/csrf';
  import { get } from 'svelte/store';
  import CrateProjectForm from '#src/lib/components/projects/CrateProjectForm.svelte';
  import Table from '#src/lib/components/Table.svelte';
  import { create } from '#src/lib/services/projects';
  import { isError } from '#src/lib/transport/ErrorResponse.js';
  import { resolve } from '$app/paths';

  let { data } = $props();
  let projects = $state(data.data || []);

  async function createProject(name: string) {
    const csrfToken = get(csrf);
    const project = await create(csrfToken, name);

    if (isError(project)) {
      return project;
    }

    projects = [project, ...projects];
  }
</script>

{#snippet linkCell(value: string | Date, id: string)}
  <a href={resolve(`/project/${id}`)}>{value}</a>
{/snippet}

{#snippet dateCell(value: string | Date)}
  {new Date(value).toUTCString()}
{/snippet}

<CrateProjectForm onCreate={createProject} />
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
  rows={projects}
/>
