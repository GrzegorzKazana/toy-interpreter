<script>
    import { tick, onMount } from 'svelte';
    import { ENTER } from '../../../config/variables';

    export let inputRef;
    export let onSubmit;

    let text = 'start typing...';

    const handleKeyPress = e => {
        if (e.keyCode !== ENTER) return false;
        onSubmit(text);
        text = '';
    };

    onMount(() => {
        tick().then(() => tick().then(() => inputRef.focus()));
    });
</script>

<p class="command">
    <span class="command__prefix">></span>
    <input
        class="command__input"
        type="text"
        bind:this={inputRef}
        bind:value={text}
        on:keypress={handleKeyPress} />
</p>

<style lang="scss">
    @import '../../../common/styles/mixins.scss';

    .command {
        display: flex;
        flex-direction: row;
        align-items: stretch;
        justify-content: flex-start;

        margin: 0 0 4px 0;

        &__input {
            flex-grow: 1;
            background-color: transparent;

            @include text-input-reset;
            @include console-text;
        }

        &__prefix {
            min-width: 2rem;
        }
    }
</style>
