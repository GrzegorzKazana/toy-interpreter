<svelte:options immutable={true} />

<svelte:head>
    <title>toy-interpreter</title>
    <meta name="description" content="toy interpreter written in Rust, running with wasm." />
</svelte:head>

<slot />

<style>
    :global(body) {
        margin: 0;
        padding: 0;
    }

    :global(*) {
        box-sizing: border-box;
    }
</style>
