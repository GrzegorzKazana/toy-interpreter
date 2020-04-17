<script>
    import { tick, onMount } from 'svelte';
    import { wrapStorage } from '../common/utils';
    import inputHistoryStore from '../stores/inputHistoryStore.ts';
    import { ENTER, ARROW_UP, ARROW_DOWN } from '../config/variables.ts';

    export let inputRef;
    export let onSubmit;

    const inputState = inputHistoryStore(wrapStorage(sessionStorage));

    const handleKeyPress = e => {
        switch (e.keyCode) {
            case ENTER: {
                const value = $inputState.value;
                onSubmit(value);
                inputState.addEntry(value);
                return;
            }
            case ARROW_UP: {
                inputState.handleUpKey();
                e.preventDefault();
                return;
            }
            case ARROW_DOWN: {
                inputState.handleDownKey();
                return;
            }
            default:
                return false;
        }
    };

    onMount(() => {
        tick().then(() => inputRef.focus());
    });
</script>

<p class="command">
    <span class="command__prefix">></span>
    <input
        class="command__input"
        type="text"
        spellcheck="false"
        bind:this={inputRef}
        value={$inputState.value}
        on:keydown={handleKeyPress}
        on:input={e => inputState.setValue(e.target.value)} />
</p>

<style lang="scss">
    @import '../common/styles/mixins.scss';

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
            min-width: 1em;
        }
    }
</style>
