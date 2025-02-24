<script lang="ts">
  import { ModeWatcher } from "mode-watcher";

  import "../app.css";
  import HeaderNav from "$lib/components/custom/HeaderNav.svelte";
  import * as Tooltip from "$lib/components/ui/tooltip/index.js";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import AppSidebar from "$lib/components/app-sidebar.svelte";

  let { children } = $props();
</script>

<ModeWatcher />

<Tooltip.Provider>
  <Sidebar.Provider>
    <AppSidebar />

    <Sidebar.Inset>
      <HeaderNav />

      {@render children()}
    </Sidebar.Inset>
  </Sidebar.Provider>
</Tooltip.Provider>
