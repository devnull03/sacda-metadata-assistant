<script lang="ts">
  import House from "lucide-svelte/icons/house";
  import ChartLine from "lucide-svelte/icons/chart-line";
  import Package from "lucide-svelte/icons/package";
  import Package2 from "lucide-svelte/icons/package-2";
  import Settings from "lucide-svelte/icons/settings";
  import ShoppingCart from "lucide-svelte/icons/shopping-cart";
  import UsersRound from "lucide-svelte/icons/users-round";

  import * as Tooltip from "$lib/components/ui/tooltip/index.js";
</script>

<aside
  class="bg-background fixed inset-y-0 left-0 z-10 hidden w-14 flex-col border-r sm:flex"
>
  <nav class="flex flex-col items-center gap-4 px-2 py-4">
    <a
      href="##"
      class="bg-primary text-primary-foreground group flex h-9 w-9 shrink-0 items-center justify-center gap-2 rounded-full text-lg font-semibold md:h-8 md:w-8 md:text-base"
    >
      <Package2 class="h-4 w-4 transition-all group-hover:scale-110" />
      <span class="sr-only">Acme Inc</span>
    </a>
    <Tooltip.Root>
      <Tooltip.Trigger asChild let:builder>
        <a
          href="##"
          class="text-muted-foreground hover:text-foreground flex h-9 w-9 items-center justify-center rounded-lg transition-colors md:h-8 md:w-8"
          use:builder.action
          {...builder}
        >
          <House class="h-5 w-5" />
          <span class="sr-only">Dashboard</span>
        </a>
      </Tooltip.Trigger>
      <Tooltip.Content side="right">Dashboard</Tooltip.Content>
    </Tooltip.Root>
    <Tooltip.Root>
      <Tooltip.Trigger asChild let:builder>
        <a
          href="##"
          class="hover:text-foreground flex h-9 w-9 items-center justify-center rounded-lg transition-colors md:h-8 md:w-8"
          use:builder.action
          {...builder}
        >
          <ShoppingCart class="h-5 w-5" />
          <span class="sr-only">Orders</span>
        </a>
      </Tooltip.Trigger>
      <Tooltip.Content side="right">Orders</Tooltip.Content>
    </Tooltip.Root>
    <Tooltip.Root>
      <Tooltip.Trigger asChild let:builder>
        <a
          href="##"
          class="bg-accent text-accent-foreground hover:text-foreground flex h-9 w-9 items-center justify-center rounded-lg transition-colors md:h-8 md:w-8"
          use:builder.action
          {...builder}
        >
          <Package class="h-5 w-5" />
          <span class="sr-only">Products</span>
        </a>
      </Tooltip.Trigger>
      <Tooltip.Content side="right">Products</Tooltip.Content>
    </Tooltip.Root>
    <Tooltip.Root>
      <Tooltip.Trigger asChild let:builder>
        <a
          href="##"
          class="text-muted-foreground hover:text-foreground flex h-9 w-9 items-center justify-center rounded-lg transition-colors md:h-8 md:w-8"
          use:builder.action
          {...builder}
        >
          <UsersRound class="h-5 w-5" />
          <span class="sr-only">Customers</span>
        </a>
      </Tooltip.Trigger>
      <Tooltip.Content side="right">Customers</Tooltip.Content>
    </Tooltip.Root>
    <Tooltip.Root>
      <Tooltip.Trigger asChild let:builder>
        <a
          href="##"
          class="text-muted-foreground hover:text-foreground flex h-9 w-9 items-center justify-center rounded-lg transition-colors md:h-8 md:w-8"
          use:builder.action
          {...builder}
        >
          <ChartLine class="h-5 w-5" />
          <span class="sr-only">Analytics</span>
        </a>
      </Tooltip.Trigger>
      <Tooltip.Content side="right">Analytics</Tooltip.Content>
    </Tooltip.Root>
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
