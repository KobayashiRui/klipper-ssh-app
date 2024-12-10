import {type Writable, writable} from 'svelte/store';

export const connected: Writable<boolean> = writable(false);

