import { writable } from 'svelte/store';

const initialState = {
    history: [] as string[],
    relevantHistory: [] as string[],
    viewedHistoryIndex: -1,
    stashedInput: '',
    value: '',
};

export default function inputHistoryStore() {
    const { subscribe, update } = writable(initialState);

    const handleUpKey = () =>
        update(s => {
            const nextHistoryIndex = Math.min(
                s.viewedHistoryIndex + 1,
                s.relevantHistory.length - 1,
            );

            return {
                ...s,
                stashedInput: s.viewedHistoryIndex === -1 ? s.value : s.stashedInput,
                viewedHistoryIndex: nextHistoryIndex,
                value: nextHistoryIndex === -1 ? s.value : s.relevantHistory[nextHistoryIndex],
            };
        });

    const handleDownKey = () =>
        update(s => {
            const nextHistoryIndex = Math.max(s.viewedHistoryIndex - 1, -1);

            return {
                ...s,
                viewedHistoryIndex: nextHistoryIndex,
                value:
                    nextHistoryIndex === -1 ? s.stashedInput : s.relevantHistory[nextHistoryIndex],
            };
        });

    const addEntry = (input: string) =>
        update(s => ({
            ...s,
            history: input ? [input, ...s.history] : s.history,
            relevantHistory: input ? [input, ...s.history] : s.history,
            value: '',
            stashedInput: '',
            viewedHistoryIndex: -1,
        }));

    const setValue = (value: string) =>
        update(s => ({
            ...s,
            relevantHistory: s.history.filter(entry => entry.startsWith(value)),
            viewedHistoryIndex: -1,
            value,
        }));

    return {
        subscribe,
        handleUpKey,
        handleDownKey,
        addEntry,
        setValue,
    };
}
