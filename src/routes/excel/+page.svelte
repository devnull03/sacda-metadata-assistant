<script lang="ts">
  import ChatArea from "$lib/components/custom/ChatArea.svelte";
  import ExcelTable from "$lib/components/custom/ExcelTable.svelte";
  import { Badge } from "$lib/components/ui/badge";
  import { PanelLeftClose } from "lucide-svelte";

  let chatCollapsed = $state(false);
</script>

<main class="grid flex-1 gap-4 overflow-auto p-4 md:grid-cols-6 h-full">
  <div class="relative hidden flex-col items-start gap-8 md:flex"
  class:md:col-span-4={!chatCollapsed}
  class:md:col-span-6={chatCollapsed}
  class:mr-6={chatCollapsed}
  >
    <ExcelTable />
  </div>

  {#if !chatCollapsed}
    <ChatArea class="md:col-span-2" bind:collapsed={chatCollapsed} />
  {:else}
    <Badge
      variant="outline"
      onclick={() => (chatCollapsed = !chatCollapsed)}
      class="hover:bg-accent p-1 absolute right-3 cursor-pointer"
    >
      <PanelLeftClose class="size-4" />
    </Badge>
  {/if}
</main>
