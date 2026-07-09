<script lang="ts">
  import type { PageProps } from './$types';
  let { data }: PageProps = $props();
  let date = new Date(data.created_at!).toUTCString();
</script>

<div class="font-special">Project details</div>
<div>Name: {data.name}</div>
<div>Created At: {date}</div>
