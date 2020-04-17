<script>
    import { tick, onMount, onDestroy } from 'svelte';
    import commandLineStore from '../CommandLineStore.ts';
    import CommandLineList from '../components/CommandLineList.svelte';

    let interpreter = null;
    const store = commandLineStore();
    store.showInitializationMessage();

    const interpret = input => {
        if (!interpreter) return;
        store.addInputLine(input);
        input && store.addOutputLine(interpreter.interpret(input));
    };

    onMount(() => {
        import('interpreter-js')
            .then(({ InterpreterJs }) => {
                interpreter = InterpreterJs.new();
                store.showInitCompleteMessage();
            })
            .catch(e => store.showInitErrorMessage(e.message));
    });

    onDestroy(() => {
        if (!interpreter) return;
        interpreter.free();
    });

    $: console.log($store.userInputHistory);
</script>

<slot
    {interpret}
    lines={$store.lines}
    inputHistory={$store.userInputHistory}
    isInitialized={!!interpreter} />
