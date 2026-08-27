<script lang="ts">
  import { resolve } from '$app/paths';
  import { auth } from '@/lib/stores/auth';
</script>

<header class="sticky top-0 py-2">
  <nav>
    {#if $auth.user}
      <span>$: {$auth.user}</span>
      <span>
        <button class="cursor-pointer" type="button" onclick={auth.logout}> (Logout) </button>
      </span>
    {:else}
      <a href={resolve('/signup')}>Sign up</a>/<a href={resolve('/signin')}>Sign in</a>
    {/if}
  </nav>
</header>
