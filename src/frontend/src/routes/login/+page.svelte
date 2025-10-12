<script lang="ts">
    import {Button} from "$lib/components/ui/button/index.js";
    import {Label} from "$lib/components/ui/label/index.js";
    import {Input} from "$lib/components/ui/input/index.js";
    // noinspection ES6UnusedImports
    import * as Card from "$lib/components/ui/card/index.js";
</script>
<div
        class="w-screen h-screen flex items-center justify-center"
>
<Card.Root class="w-full max-w-sm">
    <Card.Header class="flex flex-col">
        <Card.Title>Entra nel tuo account</Card.Title>
        <Card.Description
        >Inserisci le credenziali per accedere al programma
        </Card.Description
        >
    </Card.Header>
    <Card.Content>
        <form>
            <div class="flex flex-col gap-6">
                <div class="grid gap-2">
                    <Label for="email">Email</Label>
                    <Input id="email" type="email" placeholder="mail@itismeucci.com" required/>
                </div>
                <div class="grid gap-2">
                    <div class="flex items-center">
                        <Label for="password">Password</Label>
                        <a
                                href="##"
                                class="ml-auto inline-block text-sm underline-offset-4 hover:underline"
                        >
                            Forgot your password?
                        </a>
                    </div>
                    <Input id="password" type="password" placeholder="Password" required/>
                </div>
            </div>
        </form>
    </Card.Content>
    <Card.Footer class="flex-col gap-2">
        <Button type="submit" class="w-full">Login</Button>
    </Card.Footer>
</Card.Root>

</div>