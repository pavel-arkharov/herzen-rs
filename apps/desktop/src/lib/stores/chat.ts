import { writable, derived } from 'svelte/store';
import { onEvent, fetchApi } from './connection.js';

export type InputMethod = 'voice' | 'text';

export interface ChatTurn {
	id: string;
	role: 'user' | 'assistant';
	content: string;
	timestamp: number;
	method?: InputMethod;
	model?: string;
	stt_model?: string;
	stt_ms?: number;
	skill?: string;
	match_method?: string;
	confidence?: number;
	tts_provider?: string;
	tts_style?: string;
	tts_ms?: number;
	llm_ms?: number;
	tokens?: number;
	tok_per_s?: number;
	streaming?: boolean;
}

export const turns = writable<ChatTurn[]>([]);
export const inputMode = writable<InputMethod>('text');
export const selectedModel = writable<string>('');
export const isStreaming = writable(false);

let currentStreamTurn: string | null = null;

onEvent('turn_start', (e) => {
	const turn: ChatTurn = {
		id: e.turn_id as string,
		role: 'user',
		content: '',
		timestamp: Date.now(),
		method: e.method as InputMethod,
	};
	turns.update((t) => [...t, turn]);
});

onEvent('stt_result', (e) => {
	turns.update((t) =>
		t.map((turn) =>
			turn.id === e.turn_id
				? { ...turn, content: e.text as string, stt_model: e.model as string, stt_ms: e.duration_ms as number }
				: turn
		)
	);
});

onEvent('llm_token', (e) => {
	const turnId = e.turn_id as string;
	if (currentStreamTurn !== turnId) {
		currentStreamTurn = turnId;
		const assistantTurn: ChatTurn = {
			id: turnId + '-reply',
			role: 'assistant',
			content: '',
			timestamp: Date.now(),
			streaming: true,
		};
		turns.update((t) => [...t, assistantTurn]);
		isStreaming.set(true);
	}
	turns.update((t) =>
		t.map((turn) =>
			turn.id === turnId + '-reply'
				? { ...turn, content: turn.content + (e.token as string) }
				: turn
		)
	);
});

onEvent('llm_done', (e) => {
	const turnId = e.turn_id as string;
	turns.update((t) =>
		t.map((turn) =>
			turn.id === turnId + '-reply'
				? {
						...turn,
						streaming: false,
						model: e.model as string,
						tokens: e.tokens as number,
						llm_ms: e.duration_ms as number,
						tok_per_s: Math.round(((e.tokens as number) / (e.duration_ms as number)) * 1000),
					}
				: turn
		)
	);
	currentStreamTurn = null;
	isStreaming.set(false);
});

onEvent('skill_match', (e) => {
	const turnId = e.turn_id as string;
	const results = e.results as Array<{
		skill: string;
		method: string;
		confidence: number;
	}>;
	if (results?.length) {
		const best = results[0];
		turns.update((t) =>
			t.map((turn) =>
				turn.id === turnId + '-reply'
					? { ...turn, skill: best.skill, match_method: best.method, confidence: best.confidence }
					: turn
			)
		);
	}
});

onEvent('tts_done', (e) => {
	const turnId = e.turn_id as string;
	turns.update((t) =>
		t.map((turn) =>
			turn.id === turnId + '-reply'
				? { ...turn, tts_ms: e.duration_ms as number }
				: turn
		)
	);
});

export async function sendMessage(content: string, model?: string) {
	const turnId = crypto.randomUUID();
	const userTurn: ChatTurn = {
		id: turnId,
		role: 'user',
		content,
		timestamp: Date.now(),
		method: 'text',
	};
	turns.update((t) => [...t, userTurn]);

	try {
		const res = await fetchApi<{ content: string; model: string; prompt_tokens: number; completion_tokens: number }>(
			'/api/chat',
			{
				method: 'POST',
				body: JSON.stringify({ messages: [{ role: 'user', content }], model: model || undefined }),
			}
		);
		const replyTurn: ChatTurn = {
			id: turnId + '-reply',
			role: 'assistant',
			content: res.content,
			timestamp: Date.now(),
			model: res.model,
			tokens: res.completion_tokens,
		};
		turns.update((t) => [...t, replyTurn]);
	} catch (err) {
		const errorTurn: ChatTurn = {
			id: turnId + '-error',
			role: 'assistant',
			content: `Error: ${err instanceof Error ? err.message : 'Unknown error'}`,
			timestamp: Date.now(),
		};
		turns.update((t) => [...t, errorTurn]);
	}
}
