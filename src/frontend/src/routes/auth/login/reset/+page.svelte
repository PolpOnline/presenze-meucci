<script>
import {Input} from "$components/ui/input/index.js";
import {Button} from "$lib/components/ui/button/index.js";

    async function reset() {

    }
</script>
<main class="h-screen-not-navbar flex flex-col items-center justify-center">
    <div class="max-w-96">
        <h1 class="mb-5 text-center text-3xl font-bold">
            Reset della Password
        </h1>
        <form on:submit={reset}>
            <label class="mt-4" for="email">Email:</label>
            <Input class="my-1" type="email" autocomplete="email" required />
            <Button class="mt-4 w-full" type="submit">
                Reset Password
            </Button>
        </form>
    </div>
</main>