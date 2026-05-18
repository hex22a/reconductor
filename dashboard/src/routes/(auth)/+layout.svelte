<script lang="ts">
  let { children } = $props();
</script>

<main class="flex flex-1 flex-col justify-center">
  {@render children()}
</main>
