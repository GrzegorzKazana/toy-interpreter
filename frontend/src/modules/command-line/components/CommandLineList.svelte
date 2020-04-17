<script>
    import { tick, onMount } from 'svelte';
    import CommandLineEntry from './CommandLineEntry.svelte';
    import CommandInput from './CommandInput.svelte';

    export let interpret;
    export let lines;

    let inputRef;
</script>

<article class="command-line" on:click={() => inputRef.focus()}>
    {#each lines as line, i (i)}
        <CommandLineEntry {...line} />
    {/each}
    <CommandInput bind:inputRef onSubmit={interpret} />
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
