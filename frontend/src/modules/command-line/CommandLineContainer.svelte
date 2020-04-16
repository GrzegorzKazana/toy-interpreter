<script>
    import { tick, onMount } from 'svelte';

    import CommandLineEntry from './components/CommandLineEntry.svelte';
    import CommandInput from './components/CommandInput.svelte';

    let inputRef;
    let lines = [{ text: 'Hello world', prefix: '>' }, { text: 'Hello fellow user' }];

    const onSubmit = text => (lines = [...lines, { text, prefix: '>' }, { text: '2' }]);
</script>

<article class="command-line" on:click={() => inputRef.focus()}>
    {#each lines as line, i (i)}
        <CommandLineEntry {...line} />
    {/each}
    <CommandInput bind:inputRef {onSubmit} />
</article>

<style lang="scss">
    @import '../../common/styles/mixins.scss';

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
