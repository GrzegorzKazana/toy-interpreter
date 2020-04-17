<script>
    import CommandLineContainer from './containers/CommandLineContainer.svelte';
    import CommandLineList from './components/CommandLineList.svelte';
</script>

<CommandLineContainer let:lines let:interpret>
    <CommandLineList {lines} {interpret} />
</CommandLineContainer>
