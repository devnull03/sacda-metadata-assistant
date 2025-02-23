<script lang="ts">
  import ChatArea from "$lib/components/custom/ChatArea.svelte";

  let chatCollapsed = $state(false);
</script>

<main
  class="grid flex-1 gap-4 overflow-auto p-4 md:grid-cols-2 lg:grid-cols-6 h-full"
>
  <div
    class="relative hidden flex-col items-start gap-8 md:flex lg:col-span-4"
  ></div>

  <ChatArea class="lg:col-span-2" bind:collapsed={chatCollapsed} />

</main>
