import { writable, derived } from 'svelte/store';

export type ConnectionStatus = 'disconnected' | 'connecting' | 'connected' | 'error';

export interface ServerEvent {
	type: string;
	[key: string]: unknown;
}

const WS_URL = 'ws://127.0.0.1:3030/ws';
const REST_URL = 'http://127.0.0.1:3030';

export const status = writable<ConnectionStatus>('disconnected');
export const lastEvent = writable<ServerEvent | null>(null);
export const restUrl = writable(REST_URL);

let ws: WebSocket | null = null;
let reconnectTimer: ReturnType<typeof setTimeout> | null = null;

const eventHandlers = new Map<string, Set<(event: ServerEvent) => void>>();

export function onEvent(type: string, handler: (event: ServerEvent) => void) {
	if (!eventHandlers.has(type)) eventHandlers.set(type, new Set());
	eventHandlers.get(type)!.add(handler);
	return () => { eventHandlers.get(type)?.delete(handler); };
}

function dispatch(event: ServerEvent) {
	lastEvent.set(event);
	eventHandlers.get(event.type)?.forEach(h => h(event));
	eventHandlers.get('*')?.forEach(h => h(event));
}

export function connect(url = WS_URL) {
	if (ws && ws.readyState <= 1) return;
	status.set('connecting');

	ws = new WebSocket(url);
	ws.onopen = () => status.set('connected');
	ws.onclose = () => {
		status.set('disconnected');
		scheduleReconnect(url);
	};
	ws.onerror = () => status.set('error');
	ws.onmessage = (e) => {
		try {
			dispatch(JSON.parse(e.data));
		} catch { /* ignore malformed */ }
	};
}

export function disconnect() {
	if (reconnectTimer) clearTimeout(reconnectTimer);
	ws?.close();
	ws = null;
	status.set('disconnected');
}

export function send(data: unknown) {
	if (ws?.readyState === WebSocket.OPEN) {
		ws.send(JSON.stringify(data));
	}
}

function scheduleReconnect(url: string) {
	if (reconnectTimer) clearTimeout(reconnectTimer);
	reconnectTimer = setTimeout(() => connect(url), 3000);
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
