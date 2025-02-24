<script lang="ts">
  import { goto } from "$app/navigation";
  import * as Collapsible from "$lib/components/ui/collapsible/index.js";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import { tabs } from "$lib/utils/nav.store";
  import ChevronRight from "lucide-svelte/icons/chevron-right";
  import File from "lucide-svelte/icons/file";
  import Folder from "lucide-svelte/icons/folder";
  import type { ComponentProps } from "svelte";

  let {
    ref = $bindable(null),
    ...restProps
  }: ComponentProps<typeof Sidebar.Root> = $props();

  let flatenedTabs = $derived(Object.values(tabs));

  // This is sample data.
  const data = {
    tree: [
      ["lib", ["components", "button.svelte", "card.svelte"], "utils.ts"],
      [
        "routes",
        ["hello", "+page.svelte", "+page.ts"],
        "+page.svelte",
        "+page.server.ts",
        "+layout.svelte",
      ],
      ["static", "favicon.ico", "svelte.svg"],
      "eslint.config.js",
      ".gitignore",
      "svelte.config.js",
      "tailwind.config.js",
      "package.json",
      "README.md",
    ],
  };
</script>

<Sidebar.Root bind:ref {...restProps}>
  <Sidebar.Content>
    <Sidebar.Group>
      <Sidebar.GroupLabel>Navigation</Sidebar.GroupLabel>
      <Sidebar.GroupContent>
        <Sidebar.Menu>
          {#each flatenedTabs as item, index (index)}
            <Sidebar.MenuItem>
              <Sidebar.MenuButton onclick={() => goto(item.path)}>
                <item.component class="h-5 w-5" />
                {item.title}
              </Sidebar.MenuButton>
              <!-- <Sidebar.MenuBadge>{item.state}</Sidebar.MenuBadge> -->
            </Sidebar.MenuItem>
          {/each}
        </Sidebar.Menu>
      </Sidebar.GroupContent>
    </Sidebar.Group>
    <Sidebar.Group>
      <Sidebar.GroupLabel>Files</Sidebar.GroupLabel>
      <Sidebar.GroupContent>
        <Sidebar.Menu>
          {#each data.tree as item, index (index)}
            {@render Tree({ item })}
          {/each}
        </Sidebar.Menu>
      </Sidebar.GroupContent>
    </Sidebar.Group>
  </Sidebar.Content>
  <Sidebar.Rail />
</Sidebar.Root>

<!-- eslint-disable-next-line @typescript-eslint/no-explicit-any -->
{#snippet Tree({ item }: { item: string | any[] })}
  {@const [name, ...items] = Array.isArray(item) ? item : [item]}
  {#if !items.length}
    <Sidebar.MenuButton
      isActive={name === "button.svelte"}
      class="data-[active=true]:bg-transparent"
    >
      <File />
      {name}
    </Sidebar.MenuButton>
  {:else}
    <Sidebar.MenuItem>
      <Collapsible.Root
        class="group/collapsible [&[data-state=open]>button>svg:first-child]:rotate-90"
        open={name === "lib" || name === "components"}
      >
        <Collapsible.Trigger>
          {#snippet child({ props })}
            <Sidebar.MenuButton {...props}>
              <ChevronRight className="transition-transform" />
              <Folder />
              {name}
            </Sidebar.MenuButton>
          {/snippet}
        </Collapsible.Trigger>
        <Collapsible.Content>
          <Sidebar.MenuSub>
            {#each items as subItem, index (index)}
              {@render Tree({ item: subItem })}
            {/each}
          </Sidebar.MenuSub>
        </Collapsible.Content>
      </Collapsible.Root>
    </Sidebar.MenuItem>
  {/if}
{/snippet}
