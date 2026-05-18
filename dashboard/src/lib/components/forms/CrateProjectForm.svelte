<script lang="ts">
  let name = $state<string>('');
  let isInFlight = $state<boolean>(false);
  async function handleSubmit(e: Event) {
    e.preventDefault();
    isInFlight = true;
    console.log(name);
    name = '';
    isInFlight = false;
  }
</script>

<form onsubmit={handleSubmit}>
  <h1 class="font-special">Create project</h1>
  <fieldset class="flex w-full items-center gap-3">
    <label for="name">Project name:</label>
    <input
      class="border-0 border-b border-white bg-transparent p-2 focus:ring-0"
      id="name"
      name="name"
      type="text"
      bind:value={name}
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
