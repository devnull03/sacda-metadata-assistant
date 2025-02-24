<script lang="ts">
  import { Badge } from "$lib/components/ui/badge";
  import { Button } from "$lib/components/ui/button";
  import { Label } from "$lib/components/ui/label";
  import { Textarea } from "$lib/components/ui/textarea";
  import { CornerDownLeft, Mic, PanelRightClose, Paperclip } from "lucide-svelte";
  import * as Tooltip from "$lib/components/ui/tooltip/index.js";


  interface Props {
    collapsed?: boolean;
    class?: string;
  }

  let { collapsed = $bindable(), class: className }: Props = $props();

</script>

<div
  class="bg-muted/50 relative flex h-[85vh] min-h-[50vh] flex-col rounded-xl p-4 {className}"
>
  <Badge variant="outline" onclick={() => collapsed = !collapsed} class="hover:bg-accent p-1 absolute left-3 top-3 cursor-pointer" >
    <PanelRightClose class="size-4" />
  </Badge>
  <Badge variant="outline" class="absolute right-3 top-3">Output</Badge>
  <div class="flex-1"></div>
  <form
    class="bg-background focus-within:ring-ring relative overflow-hidden rounded-lg border focus-within:ring-1"
  >
    <Label for="message" class="sr-only">Message</Label>
    <Textarea
      id="message"
      placeholder="Type your message here..."
      class="min-h-12 resize-none border-0 p-3 shadow-none focus-visible:ring-0"
    />
    <div class="flex items-center p-3 pt-0">
      <Tooltip.Root>
        <Tooltip.Trigger asChild >
          {#snippet children({ builder })}
                    <Button builders={[builder]} variant="ghost" size="icon">
              <Paperclip class="size-4" />
              <span class="sr-only">Attach file</span>
            </Button>
                            {/snippet}
                </Tooltip.Trigger>
        <Tooltip.Content side="top">Attach File</Tooltip.Content>
      </Tooltip.Root>
      <Tooltip.Root>
        <Tooltip.Trigger asChild>
          <Button variant="ghost" size="icon">
            <Mic class="size-4" />
            <span class="sr-only">Use Microphone</span>
          </Button>
        </Tooltip.Trigger>
        <Tooltip.Content side="top">Use Microphone</Tooltip.Content>
      </Tooltip.Root>
      <Button type="submit" size="sm" class="ml-auto gap-1.5">
        Send Message
        <CornerDownLeft class="size-3.5" />
      </Button>
    </div>
  </form>
</div>
