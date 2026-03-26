<script lang="ts">
	import { models, loadedModels, memory, herzenMemoryMb, refreshModels } from '$lib/stores/models.js';
	import MemoryBar from '$lib/components/MemoryBar.svelte';
	import { onMount } from 'svelte';

	onMount(() => { refreshModels(); });

	let loaded = $derived($loadedModels);
	let available = $derived($models.filter((m) => !m.loaded));
</script>

<div class="models-page">
	<div class="models-header">
		<h2>Models</h2>
		<div class="header-right">
			<MemoryBar />
		</div>
	</div>

	<section class="model-section">
		<h3>Loaded</h3>
		{#if loaded.length === 0}
			<p class="empty">No models loaded. Start herzen-daemon with models configured.</p>
		{/if}
		{#each loaded as model (model.name)}
			<div class="model-card">
				<div class="model-card-header">
					<span class="model-name">{model.name}</span>
					<span class="badge badge-success">LOADED</span>
				</div>
				<div class="model-info">
					<span>Role: {model.role}</span>
					{#if model.path}<span class="model-path">{model.path}</span>{/if}
				</div>
				<div class="model-stats">
					<span>Memory: {model.memory_mb >= 1024 ? (model.memory_mb / 1024).toFixed(1) + ' GB' : model.memory_mb + ' MB'}</span>
					<span>GPU layers: {model.gpu_layers}</span>
					<span>Ctx: {model.context_size}</span>
					{#if model.throughput > 0}
						<span>Throughput: {model.throughput} tok/s</span>
					{/if}
				</div>
				<div class="model-sliders">
					<label class="slider-row">
						<span>Temperature</span>
						<input type="range" min="0" max="2" step="0.05" bind:value={model.temperature} />
						<span class="slider-value">{model.temperature.toFixed(2)}</span>
					</label>
					<label class="slider-row">
						<span>Top-P</span>
						<input type="range" min="0" max="1" step="0.05" bind:value={model.top_p} />
						<span class="slider-value">{model.top_p.toFixed(2)}</span>
					</label>
					<label class="slider-row">
						<span>Max tokens</span>
						<input type="range" min="64" max="4096" step="64" bind:value={model.max_tokens} />
						<span class="slider-value">{model.max_tokens}</span>
					</label>
				</div>
				<div class="model-actions">
					<button class="btn btn-outline btn-sm">Unload</button>
					<button class="btn btn-outline btn-sm">Set Default</button>
				</div>
			</div>
		{/each}
	</section>

	<section class="model-section">
		<h3>Available</h3>
		{#if available.length === 0}
			<p class="empty">No additional models found in models directory.</p>
		{/if}
		{#each available as model (model.name)}
			<div class="model-row">
				<span class="model-name">{model.name}</span>
				<span class="model-size">{model.memory_mb >= 1024 ? (model.memory_mb / 1024).toFixed(1) + ' GB' : model.memory_mb + ' MB'}</span>
				<span class="model-role">{model.role}</span>
				<button class="btn btn-outline btn-sm">Load</button>
			</div>
		{/each}
	</section>
</div>

<style>
	.models-page { display: flex; flex-direction: column; gap: 20px; }
	.models-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		gap: 24px;
	}
	.models-header h2 { font-size: 16px; font-weight: 600; flex-shrink: 0; }
	.header-right { flex: 1; max-width: 400px; }

	.model-section h3 {
		font-size: 13px;
		font-weight: 600;
		color: var(--text-secondary);
		text-transform: uppercase;
		letter-spacing: 0.5px;
		margin-bottom: 10px;
	}

	.model-card {
		background: var(--bg-surface);
		border: 1px solid var(--border);
		border-radius: var(--radius-lg);
		padding: 14px 16px;
		margin-bottom: 10px;
	}
	.model-card-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 8px;
	}
	.model-name { font-weight: 600; font-family: var(--font-mono); font-size: 14px; }

	.model-info {
		display: flex;
		gap: 16px;
		font-size: 12px;
		color: var(--text-secondary);
		margin-bottom: 8px;
	}
	.model-path {
		font-family: var(--font-mono);
		font-size: 11px;
		color: var(--text-muted);
	}

	.model-stats {
		display: flex;
		gap: 16px;
		font-size: 12px;
		color: var(--text-secondary);
		margin-bottom: 12px;
	}

	.model-sliders {
		display: flex;
		flex-direction: column;
		gap: 6px;
		margin-bottom: 12px;
	}
	.slider-row {
		display: flex;
		align-items: center;
		gap: 10px;
		font-size: 12px;
		color: var(--text-secondary);
	}
	.slider-row span:first-child { width: 90px; flex-shrink: 0; }
	.slider-row input[type="range"] { flex: 1; accent-color: var(--accent); }
	.slider-value {
		width: 50px;
		text-align: right;
		font-family: var(--font-mono);
		font-size: 12px;
	}

	.model-actions {
		display: flex;
		justify-content: flex-end;
		gap: 8px;
	}

	.model-row {
		display: flex;
		align-items: center;
		gap: 16px;
		padding: 8px 12px;
		background: var(--bg-surface);
		border: 1px solid var(--border);
		border-radius: var(--radius);
		margin-bottom: 6px;
	}
	.model-row .model-name { flex: 1; font-size: 13px; }
	.model-size, .model-role {
		font-size: 12px;
		color: var(--text-secondary);
	}

	.empty { color: var(--text-muted); font-size: 12px; }
</style>
