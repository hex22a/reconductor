<script lang="ts">
  import { page } from '$app/state';
  import { scansStore } from './scans.svelte';

  let target = $state<string>('');
  let schedule = $state<string>('');
  let isInFlight = $state<boolean>(false);
  async function handleSubmit(e: Event) {
    e.preventDefault();
    isInFlight = true;
    await scansStore.add(page.params.id!, target, schedule);
    isInFlight = false;
  }
</script>

<form onsubmit={handleSubmit}>
  <h1 class="font-special">Create scan</h1>
  <fieldset>
    <label for="target">Target:</label>
    <input
      class="border-0 border-b border-white bg-transparent p-2 focus:ring-0"
      id="target"
      name="target"
      typeof="text"
      placeholder="192.168.50.0/16"
      bind:value={target}
    />
    <label for="schedule">Schedule:</label>
    <input
      class="border-0 border-b border-white bg-transparent p-2 focus:ring-0"
      id="schedule"
      name="schedule"
      typeof="text"
      placeholder="10 5 * * *"
      bind:value={schedule}
    />
    <button
      class="cursor-pointer rounded-lg border border-white p-2 hover:bg-white hover:text-black"
      type="submit"
      disabled={isInFlight}
    >
      Create
    </button>
  </fieldset>
</form>
