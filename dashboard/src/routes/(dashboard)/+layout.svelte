<script lang="ts">
  import { goto } from '$app/navigation';
  import { resolve } from '$app/paths';
  import { auth } from '@/lib/stores/auth';
  console.log('authis', $auth, $auth != null);
  $effect(() => {
    if ($auth.ready && !$auth.user) {
      goto(resolve('/signin'));
    }
  });
  let { children } = $props();
</script>

<main class="flex flex-1 flex-col">
  {@render children()}
</main>
