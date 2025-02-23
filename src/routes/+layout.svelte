<script lang="ts">
  import { ModeWatcher } from "mode-watcher";

  import "../app.css";
  import LeftNav from "$lib/components/custom/LeftNav.svelte";
  import HeaderNav from "$lib/components/custom/HeaderNav.svelte";

  let { children } = $props();
</script>

<ModeWatcher />

<div class="bg-muted/40 flex min-h-screen h-full w-full flex-col">
  <LeftNav />

  <div class="flex flex-col sm:gap-4 sm:py-4 sm:pl-14 h-full">
    <HeaderNav />

    <main class="grid flex-1 items-start gap-4 p-4 sm:px-6 sm:py-0 md:gap-8 h-max">
      {@render children()}
    </main>
  </div>
</div>
