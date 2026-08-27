<script lang="ts">
  import { auth } from '@/lib/stores/auth';
  import FormError from './FormError.svelte';
  import { isError, type ValidationError } from '@/lib/transport/ErrorResponse';
  import { VALIDATION_ERROR_CODE } from '@/constants';
  import FieldErrors from './FieldErrors.svelte';
  import { goto } from '$app/navigation';
  import { resolve } from '$app/paths';

  let usernameErrors = $state<[string] | null>(null);
  let passwordErrors = $state<[string] | null>(null);
  let genericError = $state<string | null>(null);

  function dismissUsernameErrors() {
    usernameErrors = null;
  }

  function dismissPasswordErrors() {
    passwordErrors = null;
  }

  function dismissGenericError() {
    genericError = null;
  }

  async function handleSubmit(e: Event) {
    e.preventDefault();
    const target = e.currentTarget as HTMLFormElement;
    const formData = new FormData(target);
    const username = formData.get('username')?.toString() || '';
    const password = formData.get('password')?.toString() || '';
    const signUpResponse = await auth.signUp(username, password);
    if (isError(signUpResponse)) {
      switch (signUpResponse.code) {
        case VALIDATION_ERROR_CODE: {
          const error = signUpResponse.error as ValidationError;
          usernameErrors = error.field_errors.username ?? null;
          passwordErrors = error.field_errors.password ?? null;
          break;
        }
        default:
          genericError = 'Something went wrong';
      }
    } else {
      goto(resolve('/signin'));
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
  {#if usernameErrors}
    <FieldErrors errors={usernameErrors} onClose={dismissUsernameErrors} />
  {/if}
  <label for="password">Password</label>
  <input
    class="border-0 border-b border-white bg-transparent p-2 focus:ring-0"
    id="password"
    name="password"
    type="password"
  />
  {#if passwordErrors}
    <FieldErrors errors={passwordErrors} onClose={dismissPasswordErrors} />
  {/if}
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
