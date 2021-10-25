import { writable } from 'svelte/store';

const socket = writable<null | WebSocket>(null);

export default socket;
