import { writable, readable, derived } from 'svelte-persistent-store/dist/local';

export const auth = writable('auth', ''); // start with no user