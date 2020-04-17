<script>
    import { tick, onMount, onDestroy } from 'svelte';
    import { grammarCmd, grammar, examplesCmd, examples } from '../../../config/variables.ts';
    import commandLineStore from '../CommandLineStore.ts';
    import CommandLineList from '../components/CommandLineList.svelte';

    let interpreter = null;
    const store = commandLineStore();
    store.showInitializationMessage();

    const interpret = input => {
        store.addInputLine(input);
        switch (input) {
            case grammarCmd: {
                store.addOutputLine(grammar);
                return;
            }
            case examplesCmd: {
                store.addOutputLine(examples);
                return;
            }
            default: {
                if (!interpreter || !input) return;
                store.addOutputLine(interpreter.interpret(input));
            }
        }
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
</script>

<slot {interpret} lines={$store.lines} isInitialized={!!interpreter} />
