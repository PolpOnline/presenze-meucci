<script>

import {ButtonGroup} from "$components/ui/button-group/index.js";
import {Button} from "$components/ui/button/index.js";
import * as Drawer from "$lib/components/ui/drawer/index.js";
import {Input} from "$components/ui/input/index.js";
</script>

<main class="flex justify-center">
    <div class="max-w-[600px] w-[100%]">
        <ButtonGroup
                class="flex items-center justify-between w-[100%] mx-auto px-4 py-2  rounded-xl shadow-sm"
        >
            <Button

                    class="rounded-9px bg-background-alt hover:bg-muted inline-flex size-12 items-center justify-center active:scale-[0.98] active:transition-all"
            >
                <img src="left-arrow.svg" alt="arrow-left" class="w-full h-full" />
            </Button>

            <h1 class="text-base font-semibold text-center text-white select-none">
                oggi
            </h1>

            <Button
                    class="rounded-9px bg-background-alt hover:bg-muted inline-flex size-12 items-center justify-center active:scale-[0.98] active:transition-all"
            >
                <img src="right-arrow.svg" alt="arrow-right" class="w-full h-full" />
            </Button>
        </ButtonGroup>


        <Drawer.Root>
            <Drawer.Trigger>
                <Button
                        variant="secondary"
                        size="icon"
                        class="fixed bottom-5 right-5 w-12 h-12 rounded-2"
                >
                    <img src="/plus.svg" alt="Open drawer" class="w-6 h-6" />
                </Button>
            </Drawer.Trigger>

            <Drawer.Content class="p-6 rounded-t-2xl border-t shadow-xl bg-background">
                <Drawer.Header class="">
                    <Drawer.Title class="text-lg font-semibold">Aggiungi assenza professore</Drawer.Title>
                </Drawer.Header>
                <Input type="email" placeholder="email" class="max-w-xs" />

                <Drawer.Footer class="flex justify-end gap-2">
                    <Button>Add</Button>
                </Drawer.Footer>
            </Drawer.Content>
        </Drawer.Root>
    </div>
</main>