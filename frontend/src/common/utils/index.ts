export const tail = <T>(arr: T[]): T | undefined =>
    arr.length > 0 ? arr[arr.length - 1] : undefined;
