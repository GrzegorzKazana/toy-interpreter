import 'expect-puppeteer';

describe('Testing user commandline', () => {
    const COMMAND_INPUT_SELECTOR = 'input.command__input';
    const COMMAND_TEXT_SELECTOR = '.entry__text';

    beforeAll(async () => {
        await page.goto('http://localhost:4000/');
        page.on('console', m => console.warn(m.text()));
    });

    it('should contain project name', async () => {
        await expect(page).toMatch('toy-interpreter');
    });

    it('should contain link to repo', async () => {
        const link = await page.$('a[href*=github]');

        expect(link).not.toBeNull();
    });

    it('should focus command input onload', async () => {
        await page.waitForSelector(COMMAND_INPUT_SELECTOR);
        const commandInput = await page.$(COMMAND_INPUT_SELECTOR);
        const focusedElement = await page.evaluate(() => document.activeElement);

        expect(focusedElement).not.toBeNull();
        expect(commandInput).not.toBeNull();
        expect(focusedElement === commandInput);
    });

    it('handles basic input "2 + 2"', async () => {
        await page.waitForSelector(COMMAND_INPUT_SELECTOR);
        await page.type(COMMAND_INPUT_SELECTOR, '2 + 2');
        await page.keyboard.press('Enter');

        const cmdOutputs = await page.$$(COMMAND_TEXT_SELECTOR);
        const lastCmdOutput = cmdOutputs[cmdOutputs.length - 1];
        const outputText =
            lastCmdOutput && (await page.evaluate(el => el.textContent, lastCmdOutput));

        expect(lastCmdOutput).toBeTruthy();
        expect(outputText).toBe('4');
    });

    it('declares and executes simple function', async () => {
        await page.waitForSelector(COMMAND_INPUT_SELECTOR);
        await page.type(COMMAND_INPUT_SELECTOR, 'fun sum(a, b) = a + b');
        await page.keyboard.press('Enter');

        await page.type(COMMAND_INPUT_SELECTOR, 'sum(2, 3)');
        await page.keyboard.press('Enter');

        const cmdOutputs = await page.$$(COMMAND_TEXT_SELECTOR);
        const lastCmdOutput = cmdOutputs[cmdOutputs.length - 1];
        const outputText =
            lastCmdOutput && (await page.evaluate(el => el.textContent, lastCmdOutput));

        expect(lastCmdOutput).toBeTruthy();
        expect(outputText).toBe('5');
    });

    it('allows for scrolling input history', async () => {
        await page.waitForSelector(COMMAND_INPUT_SELECTOR);
        await page.type(COMMAND_INPUT_SELECTOR, '2 + 2');
        await page.keyboard.press('Enter');
        await page.keyboard.press('ArrowUp');

        const cmdInput = await page.$(COMMAND_INPUT_SELECTOR);
        const inputValue = cmdInput && (await page.evaluate(el => el.value, cmdInput));

        expect(cmdInput).toBeTruthy();
        expect(inputValue).toBe('2 + 2');
    });
});
