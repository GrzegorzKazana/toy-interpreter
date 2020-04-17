<script>
    import { tick, onMount } from 'svelte';
    import CommandLineEntry from './CommandLineEntry.svelte';
    import CommandInput from './CommandInput.svelte';
    import { welcomeHeader, welcomeFooter, repoUrl } from '../../../config/variables.ts';

    export let interpret;
    export let lines;
    export let isInitialized;

    let inputRef;

    const focusInput = () => inputRef.focus();
    const scrollToBottom = el => ({
        update: () => window.scrollTo(0, el.scrollHeight),
    });
</script>

<article class="command-line" on:click={focusInput} use:scrollToBottom={lines}>
    <CommandLineEntry text={welcomeHeader} />
    <a class="link" href={repoUrl}>{repoUrl}</a>
    <CommandLineEntry text={welcomeFooter} />
    {#each lines as { id, ...line } (id)}
        <CommandLineEntry {...line} />
    {/each}
    {#if isInitialized}
        <CommandInput bind:inputRef onSubmit={interpret} />
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

    .link {
        color: #fff;
    }

    :global(.command-line) {
        @include console-text;
    }
</style>
