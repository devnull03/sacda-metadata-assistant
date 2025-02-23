<script lang="ts">
  import Settings from "lucide-svelte/icons/settings";
  import * as Tooltip from "$lib/components/ui/tooltip/index.js";
  import { tabs } from "$lib/utils/nav.store";
  import { page } from "$app/state";

  let flatenedTabs = $derived(Object.values(tabs));
</script>

<aside
  class="bg-background fixed inset-y-0 left-0 z-10 hidden w-14 flex-col border-r sm:flex"
>
  <nav class="flex flex-col items-center gap-4 px-2 py-4">
    <a
      href="##"
      class="bg-primary text-primary-foreground group flex h-9 w-9 shrink-0 items-center justify-center gap-2 rounded-full text-lg font-semibold md:h-8 md:w-8 md:text-base"
    >
      <img src="/favicon.png" alt="logo" class="h-4 w-4 transition-all group-hover:scale-110" />
      <span class="sr-only">SACDA</span>
    </a>

    {#each flatenedTabs as tab}
      {@const active = tab.path === page.url.pathname}
      <Tooltip.Root>
        <Tooltip.Trigger asChild let:builder>
          <a
            href={tab.path}
            class="hover:text-foreground flex h-9 w-9 items-center justify-center rounded-lg transition-colors md:h-8 md:w-8"
            class:bg-accent={active}
            class:text-muted-foreground={!active}
            use:builder.action
            {...builder}
          >
            <tab.component class="h-5 w-5" />
            <span class="sr-only">{tab.title}</span>
          </a>
        </Tooltip.Trigger>
        <Tooltip.Content side="right">{tab.title}</Tooltip.Content>
      </Tooltip.Root>
    {/each}

  </nav>
  <nav class="mt-auto flex flex-col items-center gap-4 px-2 py-4">
    <Tooltip.Root>
      <Tooltip.Trigger asChild let:builder>
        <a
          href="##"
          class="text-muted-foreground hover:text-foreground flex h-9 w-9 items-center justify-center rounded-lg transition-colors md:h-8 md:w-8"
          use:builder.action
          {...builder}
        >
          <Settings class="h-5 w-5" />
          <span class="sr-only">Settings</span>
        </a>
      </Tooltip.Trigger>
      <Tooltip.Content side="right">Settings</Tooltip.Content>
    </Tooltip.Root>
  </nav>
</aside>
