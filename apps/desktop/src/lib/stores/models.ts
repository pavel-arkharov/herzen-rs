import { writable, derived } from 'svelte/store';
import { onEvent, fetchApi } from './connection.js';

export interface ModelStatus {
	name: string;
	role: string;
	loaded: boolean;
	memory_mb: number;
	gpu_layers: number;
	context_size: number;
	temperature: number;
	top_p: number;
	max_tokens: number;
	throughput: number;
	path?: string;
}

export interface MemoryInfo {
	system_total: number;
	system_used: number;
	herzen_breakdown: Record<string, number>;
}

export const models = writable<ModelStatus[]>([]);
export const memory = writable<MemoryInfo>({
	system_total: 16384,
	system_used: 0,
	herzen_breakdown: {},
});

export const loadedModels = derived(models, ($m) => $m.filter((m) => m.loaded));
export const herzenMemoryMb = derived(memory, ($m) =>
	Object.values($m.herzen_breakdown).reduce((a, b) => a + b, 0)
);

onEvent('model_status', (e) => {
	if (Array.isArray(e.models)) models.set(e.models as ModelStatus[]);
});

onEvent('memory_update', (e) => {
	memory.update((m) => ({
		system_total: (e.system_total as number) ?? m.system_total,
		system_used: (e.system_used as number) ?? m.system_used,
		herzen_breakdown: (e.herzen_breakdown as Record<string, number>) ?? m.herzen_breakdown,
	}));
});

export async function refreshModels() {
	try {
		const data = await fetchApi<{ models: ModelStatus[] }>('/api/models');
		models.set(data.models ?? []);
	} catch { /* server not running */ }
}
