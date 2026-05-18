<script lang="ts">
  import './layout.css';
  import favicon from '$lib/assets/favicon.svg';
  import Header from '@/lib/components/Header.svelte';
  import Footer from '@/lib/components/Footer.svelte';
  import { onMount } from 'svelte';
  import { csrf } from '@/lib/stores/csrf';
  import { auth } from '@/lib/stores/auth';

  onMount(async () => {
    await csrf.fetchCsrf();
    await auth.fetchMe();
  });

  let { children } = $props();
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>
<div class="flex min-h-screen flex-col items-center">
  <Header />
  {@render children()}
  <Footer />
</div>
