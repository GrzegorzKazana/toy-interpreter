<script>
    import CommandLineContainer from './containers/CommandLineContainer.svelte';
    import CommandLineList from './components/CommandLineList.svelte';
</script>

<CommandLineContainer let:lines let:interpret let:isInitialized let:inputHistory>
    <CommandLineList {lines} {interpret} {isInitialized} {inputHistory} />
</CommandLineContainer>
