import { writable, readable, derived } from 'svelte-persistent-store/dist/local';

export const eth = writable('eth', {
    account: null,
    patents: []
});