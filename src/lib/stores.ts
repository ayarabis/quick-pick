import { writable } from 'svelte/store';

const search = writable<string>('')

export { search }
