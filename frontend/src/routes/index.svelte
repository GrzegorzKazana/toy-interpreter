<script>
    import CommandLineContainer from '../modules/command-line/CommandLineContainer.svelte';
    import DemoContainer from '../modules/demo/DemoContainer.svelte';
</script>

<main class="page">
    <CommandLineContainer />
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
