const webpack = require('webpack');
const path = require('path');
const config = require('sapper/config/webpack.js');
const pkg = require('./package.json');
const autoPreprocess = require('svelte-preprocess');

const mode = process.env.NODE_ENV;
const dev = mode === 'development';

const alias = {
    svelte: path.resolve('node_modules', 'svelte'),
    'interpreter-js': path.resolve(__dirname, '../interpreter-js/toyinterpreter.js'),
};
const extensions = ['.mjs', '.js', '.json', '.svelte', '.html', '.ts'];
const mainFields = ['svelte', 'module', 'browser', 'main'];
const preprocess = autoPreprocess();

const ts = {
    test: /\.tsx?$/,
    use: 'ts-loader',
    exclude: /node_modules/,
};

module.exports = {
    client: {
        entry: config.client.entry(),
        output: config.client.output(),
        resolve: { alias, extensions, mainFields },
        module: {
            rules: [
                {
                    test: /\.(svelte|html)$/,
                    use: {
                        loader: 'svelte-loader',
                        options: {
                            dev,
                            hydratable: true,
                            hotReload: false, // pending https://github.com/sveltejs/svelte/issues/2377,
                            preprocess,
                        },
                    },
                },
                ts,
            ],
        },
        mode,
        plugins: [
            // pending https://github.com/sveltejs/svelte/issues/2377
            // dev && new webpack.HotModuleReplacementPlugin(),
            new webpack.DefinePlugin({
                'process.browser': true,
                'process.env.NODE_ENV': JSON.stringify(mode),
            }),
        ].filter(Boolean),
        devtool: dev && 'inline-source-map',
    },

    server: {
        entry: config.server.entry(),
        output: config.server.output(),
        target: 'node',
        resolve: { alias, extensions, mainFields },
        externals: Object.keys(pkg.dependencies).concat('encoding'),
        module: {
            rules: [
                {
                    test: /\.(svelte|html)$/,
                    use: {
                        loader: 'svelte-loader',
                        options: {
                            css: false,
                            generate: 'ssr',
                            dev,
                            preprocess,
                        },
                    },
                },
                ts,
            ],
        },
        mode: process.env.NODE_ENV,
        performance: {
            hints: false, // it doesn't matter if server.js is large
        },
    },

    serviceworker: {
        entry: config.serviceworker.entry(),
        output: config.serviceworker.output(),
        mode: process.env.NODE_ENV,
    },
};
