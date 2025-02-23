<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  let model_list = $state();

  onMount(async () => {
    const response = await invoke("list_available_models");
    model_list = response;
  });
</script>

{model_list}
