<script>
    import { tick, onMount } from 'svelte';
    import CommandLineEntry from './CommandLineEntry.svelte';
    import CommandInput from './CommandInput.svelte';
    import { welcomeHeader, welcomeFooter, repoUrl } from '../config/variables.ts';

    export let interpret;
    export let lines;
    export let isInitialized;

    let inputRef;
    let wrapperRef;

    const focusInput = e => e.target === wrapperRef && inputRef.focus();
    const scrollToBottom = el => ({
        update: () => wrapperRef.scrollTo(0, el.scrollHeight),
    });
</script>

<article
    class="command-line"
    bind:this={wrapperRef}
    on:click={focusInput}
    use:scrollToBottom={lines}>
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
    @import '../common/styles/mixins.scss';

    .command-line {
        height: 100%;
        overflow-y: scroll;
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
