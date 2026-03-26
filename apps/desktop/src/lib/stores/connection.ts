import { writable, derived } from 'svelte/store';

export type ConnectionStatus = 'disconnected' | 'connecting' | 'connected' | 'error';

export interface ServerEvent {
	type: string;
	[key: string]: unknown;
}

const REST_URL = 'http://127.0.0.1:3100';

export const status = writable<ConnectionStatus>('disconnected');
export const lastEvent = writable<ServerEvent | null>(null);

const eventHandlers = new Map<string, Set<(event: ServerEvent) => void>>();

export function onEvent(type: string, handler: (event: ServerEvent) => void) {
	if (!eventHandlers.has(type)) eventHandlers.set(type, new Set());
	eventHandlers.get(type)!.add(handler);
	return () => { eventHandlers.get(type)?.delete(handler); };
}

export function dispatch(event: ServerEvent) {
	lastEvent.set(event);
	eventHandlers.get(event.type)?.forEach(h => h(event));
	eventHandlers.get('*')?.forEach(h => h(event));
}

let pollTimer: ReturnType<typeof setInterval> | null = null;

/** Poll /health every 5s to drive connection status. WebSocket will be added later. */
export function pollHealth() {
	const check = () =>
		fetchApi('/health')
			.then(() => status.set('connected'))
			.catch(() => status.set('disconnected'));

	check();
	if (pollTimer) clearInterval(pollTimer);
	pollTimer = setInterval(check, 5000);
}

export function stopPolling() {
	if (pollTimer) { clearInterval(pollTimer); pollTimer = null; }
}

export async function fetchApi<T>(path: string, options?: RequestInit): Promise<T> {
	const res = await fetch(`${REST_URL}${path}`, {
		headers: { 'Content-Type': 'application/json' },
		...options,
	});
	if (!res.ok) throw new Error(`${res.status}: ${await res.text()}`);
	return res.json();
}

export const isConnected = derived(status, ($s) => $s === 'connected');
