<script>
    import CommandLine from '../modules/command-line/CommandLine.svelte';
    import DemoContainer from '../modules/demo/DemoContainer.svelte';
</script>

<main class="page">
    <CommandLine />
    <!-- <DemoContainer /> -->
</main>

<style lang="scss">
    .page {
        display: flex;
        flex-direction: row;
        align-items: stretch;

        width: 100vw;
        min-height: 100vh;
    }
</style>
