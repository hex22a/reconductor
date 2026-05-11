<script lang="ts">
  import { goto } from '$app/navigation';
  import { auth } from '$lib/stores/auth';
  import FormError from './FormError.svelte';

  let genericError = $state<string | null>(null);

  function dismissGenericError() {
    genericError = null;
  }

  async function handleSubmit(e: Event) {
    e.preventDefault();
    const target = e.currentTarget as HTMLFormElement;
    const formData = new FormData(target);
    const username = formData.get('username')?.toString();
    const password = formData.get('password')?.toString();
    try {
      await auth.register(username, password);
      goto('/signin');
    } catch {
      genericError = 'Something went wrong';
    }
  }
</script>

<form onsubmit={handleSubmit} class="flex w-150 flex-col gap-3 rounded-xl border border-white p-3">
  <h1 class="font-special">Sign up</h1>
  <label for="username">Username:</label>
  <input
    class="border-0 border-b border-white bg-transparent p-2 focus:ring-0"
    id="username"
    name="username"
    type="text"
  />
  <label for="password">Password</label>
  <input
    class="border-0 border-b border-white bg-transparent p-2 focus:ring-0"
    id="password"
    name="password"
    type="password"
  />
  <button
    class="cursor-pointer rounded-lg border border-white p-2 hover:bg-white hover:text-black"
    type="submit"
  >
    Sign in
  </button>
  {#if genericError}
    <FormError error={genericError} onClose={dismissGenericError} />
  {/if}
</form>
