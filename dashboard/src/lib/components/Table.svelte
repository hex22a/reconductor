<script lang="ts" generics="T extends { id: string } & Record<string, unknown>">
  import type { Snippet } from 'svelte';

  type Column = {
    key: keyof T;
    label: string;
    render?: Snippet<[T[keyof T], string]>;
  };

  type Props = {
    columns: Array<Column>;
    rows: ReadonlyArray<T>;
  };

  let { columns, rows }: Props = $props();
</script>

<table class="my-2 w-full">
  <thead class="py-2">
    <tr>
      {#each columns as column (column.key)}
        <th class="p-1 text-left">{column.label}</th>
      {/each}
    </tr>
  </thead>
  <tbody>
    {#each rows as row (row.id)}
      <tr>
        {#each columns as column (column.key)}
          <td class="p-1">
            {#if column.render}
              {@render column.render(row[column.key], row.id)}
            {:else}
              {row[column.key] ?? '---'}
            {/if}
          </td>
        {/each}
      </tr>
    {/each}
  </tbody>
</table>
