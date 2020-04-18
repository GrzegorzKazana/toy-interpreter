export const head = <T>(arr: T[]): T | undefined => arr[0];

export const tail = <T>(arr: T[]): T | undefined =>
    arr.length > 0 ? arr[arr.length - 1] : undefined;

export type WrappedStorage = {
    load: <T extends object>(key: string) => T | null;
    save: <T extends object>(key: string, item: T) => void;
};

export const wrapStorage = (storage: Storage): WrappedStorage => ({
    save: (key, item) => storage.setItem(key, JSON.stringify(item)),
    load: key => {
        try {
            const item = storage.getItem(key);
            return item ? JSON.parse(item) : null;
        } catch {
            return null;
        }
    },
});
