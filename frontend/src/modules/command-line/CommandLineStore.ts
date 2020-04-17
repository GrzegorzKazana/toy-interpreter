import { writable } from 'svelte/store';
import { v4 as uuid } from 'uuid';

type Line = {
    id: string;
    text: string;
    prefix?: string;
};

const initialState = {
    initMessageId: '',
    lines: [] as Line[],
    userInputHistory: [] as string[],
};

export default function commandLineStore() {
    const { subscribe, update } = writable(initialState);

    const addInputLine = (input: string) =>
        update(s => ({
            ...s,
            userInputHistory: input ? [input, ...s.userInputHistory] : s.userInputHistory,
            lines: [...s.lines, { id: uuid(), text: input, prefix: '>' }],
        }));

    const addOutputLine = (output: string) =>
        update(s => ({ ...s, lines: [...s.lines, { id: uuid(), text: output }] }));

    const showInitializationMessage = () =>
        update(s => {
            const initMessageId = uuid();

            return {
                ...s,
                initMessageId,
                lines: [...s.lines, { id: initMessageId, text: 'Initializing...' }],
            };
        });

    const showInitCompleteMessage = () =>
        update(s => ({
            ...s,
            lines: s.lines.map(line =>
                line.id === s.initMessageId ? { ...line, text: `${line.text} Done!` } : line,
            ),
        }));

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
