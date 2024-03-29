module.exports = {
    server: {
        command: 'npm run dev -- --port=4000',
        launchTimeout: 20000,
        port: 4000,
    },
    launch: {
        headless: true,
        args: ['--no-sandbox', '--disable-setuid-sandbox'],
    },
};
