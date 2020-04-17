<script>
    export let text = '';
    export let prefix = '';
</script>

<p class="entry">
    {#if prefix}
        <span class="entry__prefix">{prefix}</span>
    {/if}
    <span class="entry__text">{text}</span>
</p>

<style lang="scss">
    .entry {
        display: flex;
        flex-direction: row;
        align-items: stretch;
        justify-content: flex-start;

        margin: 0 0 4px 0;

        white-space: pre-line;

        &__prefix {
            min-width: 1em;
        }
    }
</style>
