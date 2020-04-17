<script>
    import { tick, onMount, onDestroy } from 'svelte';
    import commandLineStore from '../CommandLineStore.ts';
    import CommandLineList from '../components/CommandLineList.svelte';

    let interpreter = null;
    const store = commandLineStore();

    const interpret = input => {
        if (!interpreter) return;
        store.addInputLine(input);
        try {
            const output = interpreter.interpret(input);
            store.addOutputLine(output);
        } catch (msg) {
            console.warn(msg);
            store.addOutputLine(msg);
        }
    };

    onMount(() => {
        import('interpreter-js')
            .then(({ InterpreterJs }) => {
                interpreter = InterpreterJs.new();
                console.log('loaded!', interpreter);
            })
            .catch(console.warn);
    });

    onDestroy(() => {
        if (!interpreter) return;
        interpreter.free();
    });
</script>

<slot {interpret} lines={$store.lines} />
