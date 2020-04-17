import { writable } from 'svelte/store';

type Line = {
    text: string;
    prefix?: string;
};

const initialState = {
    lines: [] as Line[],
};

export default function commandLineStore() {
    const { subscribe, update } = writable(initialState);

    const addInputLine = (input: string) =>
        update(s => ({ ...s, lines: [...s.lines, { text: input, prefix: '>' }] }));

    const addOutputLine = (output: string) =>
        update(s => ({ ...s, lines: [...s.lines, { text: output }] }));

    return {
        subscribe,
        addInputLine,
        addOutputLine,
    };
}
