import { get } from 'svelte/store';
import inputHistoryStore from '../stores/inputHistoryStore';

const mockStorage = {
    load: () => null,
    save: () => {},
};

describe('input history store', () => {
    it('accepts input value', () => {
        const store = inputHistoryStore(mockStorage);
        const input = 'asd';

        store.setValue(input);

        expect(get(store).value).toBe(input);
    });

    it('handles scrolling to previous', () => {
        const store = inputHistoryStore(mockStorage);
        const input1 = 'command1';

        store.addEntry(input1);
        store.prev();

        expect(get(store).value).toBe(input1);
    });

    it('handles scrolling to previous and back to default', () => {
        const store = inputHistoryStore(mockStorage);
        const input1 = 'command1';
        const input2 = 'command2';

        store.addEntry(input1);
        store.setValue(input2);
        store.prev();
        store.next();

        expect(get(store).value).toBe(input2);
    });

    it('filters history by input', () => {
        const store = inputHistoryStore(mockStorage);
        const input1 = '1command';
        const input2 = '2command';
        const input3 = '1';

        store.addEntry(input1);
        store.addEntry(input2);
        store.setValue(input3);
        store.prev();

        expect(get(store).value).toBe(input1);
    });

    it('accepts input after scrolling to history', () => {
        const store = inputHistoryStore(mockStorage);
        const input1 = '1command';
        const input2 = '2command';
        const input3 = '3command';

        store.addEntry(input1);
        store.addEntry(input2);
        store.prev();
        store.setValue(input3);

        expect(get(store).value).toBe(input3);
    });
});
