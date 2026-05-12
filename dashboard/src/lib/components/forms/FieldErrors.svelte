<script lang="ts">
  import CrossIcon from '@/lib/icons/CrossIcon.svelte';
  let { errors, onClose } = $props();
</script>

<div class="relative rounded-xl border border-red-300 p-2 pr-5 text-red-300">
  <button onclick={onClose} class="absolute top-2 right-1 cursor-pointer">
    <CrossIcon className="fill-red-300 opacity-60 hover:opacity-80" width={24} height={24} />
  </button>
  {#each errors as error}
    <p class="border-red-300 pb-2 not-last:border-b">{error}</p>
  {/each}
</div>
