<script lang="ts">
	import { turns, sendMessage, selectedModel, inputMode, isStreaming } from '$lib/stores/chat.js';
	import { loadedModels } from '$lib/stores/models.js';
	import { status } from '$lib/stores/connection.js';
	import ChatMessage from '$lib/components/ChatMessage.svelte';

	let inputText = $state('');
	let chatEl: HTMLElement | undefined = $state();

	$effect(() => {
		if ($turns.length && chatEl) {
			chatEl.scrollTop = chatEl.scrollHeight;
		}
	});

	function handleSend() {
		const text = inputText.trim();
		if (!text || $isStreaming) return;
		sendMessage(text, $selectedModel || undefined);
		inputText = '';
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter' && !e.shiftKey) {
			e.preventDefault();
			handleSend();
		}
	}
</script>

<div class="chat-page">
	<div class="chat-header">
		<h2>Chat</h2>
		<div class="chat-controls">
			<span class="conn-status badge" class:badge-success={$status === 'connected'} class:badge-error={$status === 'error' || $status === 'disconnected'}>
				{$status}
			</span>
		</div>
	</div>

	<div class="chat-messages" bind:this={chatEl}>
		{#if $turns.length === 0}
			<div class="chat-empty">
				<p>No messages yet. Type below or use voice input.</p>
			</div>
		{/if}
		{#each $turns as turn (turn.id)}
			<ChatMessage {turn} />
		{/each}
	</div>

	<div class="chat-input-bar">
		<div class="input-row">
			<textarea
				class="chat-input"
				placeholder="Type a message..."
				bind:value={inputText}
				onkeydown={handleKeydown}
				rows={1}
				disabled={$isStreaming}
			></textarea>
			<button class="btn btn-primary" onclick={handleSend} disabled={$isStreaming || !inputText.trim()}>
				Send
			</button>
		</div>
		<div class="input-meta">
			<label class="meta-select">
				Mode:
				<select bind:value={$inputMode}>
					<option value="text">Text</option>
					<option value="voice">Voice</option>
				</select>
			</label>
			{#if $loadedModels.length > 0}
				<label class="meta-select">
					Model:
					<select bind:value={$selectedModel}>
						<option value="">Default</option>
						{#each $loadedModels as m}
							<option value={m.name}>{m.name}</option>
						{/each}
					</select>
				</label>
			{/if}
		</div>
	</div>
</div>

<style>
	.chat-page {
		display: flex;
		flex-direction: column;
		height: 100%;
		gap: 0;
	}
	.chat-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding-bottom: 12px;
		border-bottom: 1px solid var(--border);
		flex-shrink: 0;
	}
	.chat-header h2 {
		font-size: 16px;
		font-weight: 600;
	}
	.chat-controls { display: flex; gap: 8px; align-items: center; }

	.chat-messages {
		flex: 1;
		overflow-y: auto;
		display: flex;
		flex-direction: column;
		gap: 10px;
		padding: 16px 0;
	}

	.chat-empty {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--text-muted);
	}

	.chat-input-bar {
		flex-shrink: 0;
		border-top: 1px solid var(--border);
		padding-top: 12px;
	}
	.input-row {
		display: flex;
		gap: 8px;
	}
	.chat-input {
		flex: 1;
		resize: none;
		min-height: 36px;
		max-height: 120px;
		padding: 8px 12px;
		font-family: var(--font-sans);
	}
	.input-meta {
		display: flex;
		gap: 16px;
		margin-top: 8px;
		font-size: 12px;
		color: var(--text-secondary);
	}
	.meta-select {
		display: flex;
		align-items: center;
		gap: 4px;
	}
	.meta-select select {
		font-size: 12px;
		padding: 2px 6px;
	}
</style>
