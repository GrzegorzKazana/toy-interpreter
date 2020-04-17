<script>
    import { tick, onMount } from 'svelte';
    import CommandLineEntry from './CommandLineEntry.svelte';
    import CommandInput from './CommandInput.svelte';

    export let interpret;
    export let lines;
    export let inputHistory;
    export let isInitialized;

    let inputRef = undefined;
</script>

<article class="command-line" on:click={() => inputRef.focus()}>
    {#each lines as { id, ...line } (id)}
        <CommandLineEntry {...line} />
    {/each}
    {#if isInitialized}
        <CommandInput bind:inputRef onSubmit={interpret} {inputHistory} />
    {/if}
</article>

<style lang="scss">
    @import '../../../common/styles/mixins.scss';

    .command-line {
        flex-grow: 1;
        flex-basis: 0;
        background-color: #000;
        padding: 1.5rem;
    }

    :global(.command-line) {
        @include console-text;
    }
</style>
