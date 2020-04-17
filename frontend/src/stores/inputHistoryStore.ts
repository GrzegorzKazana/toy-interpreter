import { writable } from 'svelte/store';
import { head, WrappedStorage } from '../common/utils';

const COMMAND_HISTORY_KEY = 'COMMAND_HISTORY_KEY';

const initialState = {
    history: [] as string[],
    relevantHistory: [] as string[],
    viewedHistoryIndex: -1,
    stashedInput: '',
    value: '',
};

export default function inputHistoryStore(storage: WrappedStorage) {
    const history = storage.load<string[]>(COMMAND_HISTORY_KEY);
    const { subscribe, update } = writable(
        history ? { ...initialState, history, relevantHistory: history } : initialState,
    );

    const prev = () =>
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

    const next = () =>
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
        update(s => {
            const history = input && input !== head(s.history) ? [input, ...s.history] : s.history;
            storage.save(COMMAND_HISTORY_KEY, history);

            return {
                ...s,
                history,
                relevantHistory: history,
                value: '',
                stashedInput: '',
                viewedHistoryIndex: -1,
            };
        });

    const setValue = (value: string) =>
        update(s => ({
            ...s,
            relevantHistory: s.history.filter(entry => entry.startsWith(value)),
            viewedHistoryIndex: -1,
            value,
        }));

    return {
        subscribe,
        prev,
        next,
        addEntry,
        setValue,
    };
}
