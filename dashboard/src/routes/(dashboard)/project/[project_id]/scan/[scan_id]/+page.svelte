<script lang="ts">
  import type { PageProps } from './$types';
  let { data }: PageProps = $props();
  let date = new Date(data.created_at!).toUTCString();
</script>

<div class="font-special">Scan Details</div>
<div>Target: {data.target}</div>
<div>Schedle: {data.schedule}</div>
<div>Created At: {date}</div>
