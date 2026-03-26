import { writable } from 'svelte/store';
import { fetchApi } from './connection.js';

export interface SkillSlot {
	name: string;
	required: boolean;
	values: Record<string, Record<string, string[]>>;
	default?: string;
}

export interface SkillDef {
	name: string;
	description: string;
	priority: number;
	enabled: boolean;
	slots: SkillSlot[];
	semantic_examples: string[];
	semantic_threshold: number;
	actions: Array<{ type: string; [key: string]: unknown }>;
	confidence_gates: {
		auto_execute: number;
		confirm_above: number;
		reject_below: number;
		destructive: boolean;
	};
	response_template: string;
	confirm_template: string;
}

export interface MatchResult {
	skill: string;
	confidence: number;
	method: string;
	slots: Record<string, string>;
	decision: 'execute' | 'confirm' | 'reject';
}

export const skills = writable<SkillDef[]>([]);
export const selectedSkill = writable<string | null>(null);
export const testResults = writable<MatchResult[]>([]);

export async function refreshSkills() {
	try {
		const data = await fetchApi<{ skills: SkillDef[] }>('/api/skills');
		skills.set(data.skills ?? []);
	} catch { /* server not running */ }
}

export async function testInput(input: string) {
	try {
		const data = await fetchApi<{ results: MatchResult[] }>('/api/skills/test', {
			method: 'POST',
			body: JSON.stringify({ input }),
		});
		testResults.set(data.results ?? []);
	} catch { /* server not running */ }
}
