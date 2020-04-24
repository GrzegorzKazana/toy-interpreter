<script>
    import CommandLineContainer from '../containers/CommandLineContainer.svelte';
    import CommandLineList from '../components/CommandLineList.svelte';
</script>

<main class="page">
    <CommandLineContainer let:lines let:interpret let:isInitialized>
        <CommandLineList {lines} {interpret} {isInitialized} />
    </CommandLineContainer>
</main>

<style lang="scss">
    .page {
        width: 100vw;
        height: 100vh;
    }
</style>
