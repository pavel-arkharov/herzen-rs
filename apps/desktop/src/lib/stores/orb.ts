import { writable } from 'svelte/store';
import { onEvent } from './connection.js';

export type OrbState = 'idle' | 'listening' | 'processing' | 'speaking' | 'error' | 'thinking';

export const orbState = writable<OrbState>('idle');
export const audioLevel = writable(0);

onEvent('orb_state', (e) => {
	orbState.set(e.state as OrbState);
});

onEvent('audio_level', (e) => {
	audioLevel.set(e.level as number);
});
