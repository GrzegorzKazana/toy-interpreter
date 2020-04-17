import { writable } from 'svelte/store';

export default function store() {
    return writable(3);
}
