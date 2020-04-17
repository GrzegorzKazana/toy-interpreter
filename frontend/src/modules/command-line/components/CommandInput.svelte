<script>
    import { tick, onMount } from 'svelte';
    import { writable } from 'svelte/store';
    import { ENTER, ARROW_UP, ARROW_DOWN } from '../../../config/variables.ts';

    export let inputRef;
    export let onSubmit;
    export let inputHistory;

    let text = '2 + 2';
    let stashedText = '';
    let viewedHistoryIndex = -1;

    $: relevantInputHistory = inputHistory.filter(historyInput =>
        historyInput.startsWith(stashedText),
    );

    // function st() {
    //     const {subscribe, update} = writable({ history: [], stashedInput: '', value: '' })

    //     return
    // }

    const handleKeyPress = e => {
        switch (e.keyCode) {
            case ENTER: {
                onSubmit(text.trim());
                text = '';
                stashedText = '';
                viewedHistoryIndex = -1;
                return;
            }
            case ARROW_UP: {
                if (viewedHistoryIndex === -1) stashedText = text;
                viewedHistoryIndex = Math.min(
                    viewedHistoryIndex + 1,
                    relevantInputHistory.length - 1,
                );
                text = relevantInputHistory[viewedHistoryIndex];
                e.preventDefault();
                tick().then(() => {
                    console.log({ relevantInputHistory, viewedHistoryIndex });
                    inputRef.setSelectionRange(
                        relevantInputHistory[viewedHistoryIndex].length,
                        relevantInputHistory[viewedHistoryIndex].length,
                    );
                });
                return;
            }
            case ARROW_DOWN: {
                viewedHistoryIndex = Math.max(viewedHistoryIndex - 1, -1);
                stashedText = text;
                text = relevantInputHistory[viewedHistoryIndex] || '';
                return;
            }
            default:
                return false;
        }
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
        on:keydown={handleKeyPress} />
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
            min-width: 1em;
        }
    }
</style>
