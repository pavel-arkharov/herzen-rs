import { writable } from 'svelte/store';
import { onEvent } from './connection.js';

export interface ProcessInfo {
	name: string;
	status: 'running' | 'idle' | 'stopped';
	memory_mb: number;
	pid?: number;
}

export interface LatencyStats {
	stt_ms: number;
	matching_ms: number;
	llm_ms: number;
	tts_ms: number;
	total_ms: number;
}

export interface SkillActivity {
	name: string;
	triggers: number;
}

export const uptime = writable(0);
export const processes = writable<ProcessInfo[]>([]);
export const latency = writable<LatencyStats>({
	stt_ms: 0, matching_ms: 0, llm_ms: 0, tts_ms: 0, total_ms: 0,
});
export const skillActivity = writable<SkillActivity[]>([]);

onEvent('system_status', (e) => {
	if (e.uptime != null) uptime.set(e.uptime as number);
	if (Array.isArray(e.processes)) processes.set(e.processes as ProcessInfo[]);
	if (e.latency) latency.set(e.latency as LatencyStats);
	if (Array.isArray(e.skill_activity)) skillActivity.set(e.skill_activity as SkillActivity[]);
});
