<script lang="ts" generics="T extends { id: string } & Record<string, unknown>">
  import type { Snippet } from 'svelte';

  type Column = {
    key: keyof T;
    label: string;
    render?: Snippet<[T[keyof T], string]>;
  };

  type Props = {
    columns: Array<Column>;
    rows: ReadonlyArray<{ node: T }>;
  };

  let { columns, rows }: Props = $props();
</script>

<table class="my-2 w-full">
  <thead class="py-2">
    <tr>
      {#each columns as column}
        <th class="p-1 text-left">{column.label}</th>
      {/each}
    </tr>
  </thead>
  <tbody>
    {#each rows as row}
      <tr>
        {#each columns as column}
          <td class="p-1">
            {#if column.render}
              {@render column.render(row.node[column.key], row.node.id)}
            {:else}
              {row.node[column.key] ?? '---'}
            {/if}
          </td>
        {/each}
      </tr>
    {/each}
  </tbody>
</table>
