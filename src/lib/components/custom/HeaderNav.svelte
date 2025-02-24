<script lang="ts">
  import ThemeSwitcher from "$lib/components/custom/ThemeSwitcher.svelte";
  import * as Breadcrumb from "$lib/components/ui/breadcrumb/index.js";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import { Separator } from "$lib/components/ui/separator/index.js";
  import { page } from "$app/state";

  let parsedRoute = $derived(
    page.url.pathname
      .split("/")
      .map((e, i) => ({
        name: e.charAt(0).toUpperCase() + e.slice(1),
        href: page.url.pathname
          .split("/")
          .slice(0, i + 1)
          .join("/"),
      }))
      .filter((e, i) => i === 0 || e.name !== "")
  );
</script>

<header class="flex h-16 shrink-0 items-center gap-2 border-b px-4">
  <Sidebar.Trigger class="-ml-1" />
  <Separator orientation="vertical" class="mr-2 h-4" />
  <Breadcrumb.Root>
    <Breadcrumb.List>
      {#each parsedRoute as crumb, idx}
        <Breadcrumb.Item class="hidden md:block">
          {#if idx === parsedRoute.length - 1}
            <Breadcrumb.Page>{crumb.name || "Home"}</Breadcrumb.Page>
          {:else}
            <Breadcrumb.Link href={crumb.href || "/"}>
              {crumb.name || "Home"}
            </Breadcrumb.Link>
          {/if}
        </Breadcrumb.Item>
        {#if idx !== parsedRoute.length - 1}
          <Breadcrumb.Separator class="hidden md:block" />
        {/if}
      {/each}
    </Breadcrumb.List>
  </Breadcrumb.Root>

  <div class="flex-1"></div>

  <ThemeSwitcher />
</header>

<!-- 
<header
  class="bg-background sticky top-0 z-30 flex h-14 items-center gap-4 border-b px-4 sm:static sm:h-auto sm:border-0 sm:bg-transparent sm:px-6"
>
  <Sheet.Root>
    <Sheet.Trigger asChild >
      {#snippet children({ builder })}
            <Button
          builders={[builder]}
          size="icon"
          variant="outline"
          class="sm:hidden"
        >
          <PanelLeft class="h-5 w-5" />
          <span class="sr-only">Toggle Menu</span>
        </Button>
                {/snippet}
        </Sheet.Trigger>
    <Sheet.Content side="left" class="sm:max-w-xs">
      <nav class="grid gap-6 text-lg font-medium">
        <a
          href="##"
          class="bg-primary text-primary-foreground group flex h-10 w-10 shrink-0 items-center justify-center gap-2 rounded-full text-lg font-semibold md:text-base"
        >
          <img src="/favicon.png" alt="logo" class="h-5 w-5 transition-all group-hover:scale-110" />
          <span class="sr-only">SACDA</span>
        </a>

        {#each flatenedTabs as tab}
          <a
            href={tab.path}
            class="{tab.path === page.url.pathname
              ? 'text-foreground'
              : 'text-muted-foreground hover:text-foreground'} flex items-center gap-4 px-2.5"
          >
            <tab.component class="h-5 w-5" />
            {tab.title}
          </a>
        {/each}
      </nav>
    </Sheet.Content>
  </Sheet.Root>

  <Breadcrumb.Root class="hidden md:flex">
    <Breadcrumb.List>
      {#each parsedRoute as crumb, idx}
        <Breadcrumb.Item>
          {#if idx === parsedRoute.length - 1}
            <Breadcrumb.Page>{crumb.name || "Home"}</Breadcrumb.Page>
          {:else}
            <Breadcrumb.Link href={crumb.href || "/"}
              >{crumb.name || "Home"}</Breadcrumb.Link
            >
            <Breadcrumb.Separator />
          {/if}
        </Breadcrumb.Item>
      {/each}
    </Breadcrumb.List>
  </Breadcrumb.Root>

  <div class="relative ml-auto flex-1 md:grow-0">
    <Search class="text-muted-foreground absolute left-2.5 top-2.5 h-4 w-4" />
    <Input
      type="search"
      placeholder="Search..."
      class="bg-background w-full rounded-lg pl-8 md:w-[200px] lg:w-[320px]"
    />
  </div>

  <ThemeSwitcher />

  <DropdownMenu.Root>
    <DropdownMenu.Trigger asChild >
      {#snippet children({ builder })}
            <Button
          builders={[builder]}
          variant="outline"
          size="icon"
          class="overflow-hidden rounded-full"
        >
          <img
            src="/images/placeholder-user.jpg"
            width={36}
            height={36}
            alt="Avatar"
            class="overflow-hidden rounded-full"
          />
        </Button>
                {/snippet}
        </DropdownMenu.Trigger>
    <DropdownMenu.Content align="end">
      <DropdownMenu.Label>My Account</DropdownMenu.Label>
      <DropdownMenu.Separator />
      <DropdownMenu.Item>Settings</DropdownMenu.Item>
      <DropdownMenu.Item>Support</DropdownMenu.Item>
      <DropdownMenu.Separator />
      <DropdownMenu.Item>Logout</DropdownMenu.Item>
    </DropdownMenu.Content>
  </DropdownMenu.Root>
</header> 
-->
