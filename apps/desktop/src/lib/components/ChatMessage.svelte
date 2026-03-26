<script lang="ts">
	import type { ChatTurn } from '$lib/stores/chat.js';

	let { turn }: { turn: ChatTurn } = $props();
</script>

<div class="message" class:user={turn.role === 'user'} class:assistant={turn.role === 'assistant'}>
	<div class="message-header">
		<span class="message-role">{turn.role === 'user' ? 'You' : 'Herzen'}</span>
	</div>
	<div class="message-content">
		{turn.content}
		{#if turn.streaming}
			<span class="cursor">▊</span>
		{/if}
	</div>
	<div class="message-meta">
		{#if turn.method === 'voice' && turn.stt_model}
			<span class="meta-tag">🎤 {turn.stt_model} · {turn.stt_ms}ms</span>
		{:else if turn.method === 'text'}
			<span class="meta-tag">⌨️ text</span>
		{/if}
		{#if turn.skill}
			<span class="meta-tag">⚡ {turn.skill} · {turn.match_method} · {turn.confidence?.toFixed(2)}</span>
		{/if}
		{#if turn.model}
			<span class="meta-tag">🧠 {turn.model}{#if turn.llm_ms} · {(turn.llm_ms / 1000).toFixed(1)}s{/if}{#if turn.tok_per_s} · {turn.tok_per_s} tok/s{/if}</span>
		{/if}
		{#if turn.tts_provider}
			<span class="meta-tag">🔊 {turn.tts_provider}{#if turn.tts_style} · {turn.tts_style}{/if}{#if turn.tts_ms} · {turn.tts_ms}ms{/if}</span>
		{/if}
	</div>
</div>

<style>
	.message {
		padding: 10px 14px;
		border-radius: var(--radius-lg);
		background: var(--bg-surface);
		border: 1px solid var(--border);
	}

	.message-header {
		margin-bottom: 4px;
	}
	.message-role {
		font-size: 11px;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		color: var(--text-secondary);
	}
	.user .message-role { color: var(--info); }
	.assistant .message-role { color: var(--accent); }

	.message-content {
		line-height: 1.6;
		white-space: pre-wrap;
		word-wrap: break-word;
	}

	.cursor {
		animation: blink 0.8s step-end infinite;
		color: var(--accent);
	}
	@keyframes blink {
		50% { opacity: 0; }
	}

	.message-meta {
		display: flex;
		flex-wrap: wrap;
		gap: 8px;
		margin-top: 6px;
	}
	.meta-tag {
		font-size: 11px;
		color: var(--text-muted);
	}
</style>
