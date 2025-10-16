<script lang="ts">
    // noinspection ES6UnusedImports
    import * as Card from "$lib/components/ui/card";
    import {Button} from "$lib/components/ui/button";
    import {Label} from "$lib/components/ui/label";
    import {Input} from "$lib/components/ui/input";
    // import { StatusCodes } from 'http-status-codes';
    import { client } from '$lib/api/api';
    // import {redirect} from "@sveltejs/kit";

    let mail = '';
    let password = '';

    console.log(client);

    async function login() {

        console.log("Login iniziato!");

        await client.POST('/login', {
            body: {
                username: mail,
                password: password,
            },
        });
    }
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
            <form method="POST" action="/src/frontend/static">
                <div class="flex flex-col gap-6">
                    <div class="grid gap-2">
                        <Label for="email">Email</Label>
                        <Input id="email" type="email" placeholder="mail@itismeucci.com" bind:value={mail} required/>
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
                        <Input id="password" type="password" placeholder="Password" bind:value={password} required/>
                    </div>
                </div>
            </form>
        </Card.Content>
        <Card.Footer class="flex-col gap-2">
            <Button type="submit" class="w-full" onclick={login}>Login</Button>
        </Card.Footer>
    </Card.Root>

</div>