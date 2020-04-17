import { writable } from 'svelte/store';
import { v4 as uuid } from 'uuid';

type Line = {
    id: string;
    text: string;
    prefix?: string;
};

const initialState = {
    lines: [] as Line[],
};

export default function commandLineStore() {
    const { subscribe, update } = writable(initialState);

    const addInputLine = (input: string) =>
        update(s => ({
            ...s,
            lines: [...s.lines, { id: uuid(), text: input, prefix: '>' }],
        }));

    const addOutputLine = (output: string) =>
        update(s => ({ ...s, lines: [...s.lines, { id: uuid(), text: output }] }));

    const showInitializationMessage = () => addOutputLine('Initializing...');
    const showInitCompleteMessage = () => addOutputLine('Done!');
    const showInitErrorMessage = (err: string) => addOutputLine(`Error occured: ${err}`);

    return {
        subscribe,
        addInputLine,
        addOutputLine,
        showInitializationMessage,
        showInitCompleteMessage,
        showInitErrorMessage,
    };
}
