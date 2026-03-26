<script lang="ts">
	import { uptime, processes, latency, skillActivity } from '$lib/stores/system.js';
	import { memory, herzenMemoryMb, loadedModels } from '$lib/stores/models.js';
	import MemoryBar from '$lib/components/MemoryBar.svelte';

	let uptimeStr = $derived(formatUptime($uptime));

	function formatUptime(seconds: number): string {
		if (seconds === 0) return '—';
		const h = Math.floor(seconds / 3600);
		const m = Math.floor((seconds % 3600) / 60);
		return h > 0 ? `${h}h ${m}m` : `${m}m`;
	}

	function barWidth(ms: number, max: number): string {
		return Math.min((ms / max) * 100, 100) + '%';
	}
</script>

<div class="system-page">
	<div class="system-header">
		<h2>System</h2>
		<span class="uptime">Uptime: {uptimeStr}</span>
	</div>

	<!-- Memory -->
	<section class="panel">
		<h3>Memory</h3>
		<div class="panel-body">
			<div class="mem-overview">
				<div class="mem-row">
					<span>System</span>
					<span>{($memory.system_used / 1024).toFixed(1)} / {($memory.system_total / 1024).toFixed(0)} GB</span>
				</div>
				<div class="bar-track">
					<div class="bar-fill" style:width="{($memory.system_used / $memory.system_total * 100)}%"></div>
				</div>
			</div>
			<div class="herzen-mem">
				<div class="mem-row">
					<span>Herzen total</span>
					<span>{($herzenMemoryMb / 1024).toFixed(1)} GB</span>
				</div>
				{#each $loadedModels as m}
					<div class="mem-item">
						<span class="mem-item-name">{m.name}</span>
						<div class="bar-track bar-sm">
							<div class="bar-fill accent" style:width="{Math.min(m.memory_mb / 3000 * 100, 100)}%"></div>
						</div>
						<span class="mem-item-val">{m.memory_mb >= 1024 ? (m.memory_mb / 1024).toFixed(1) + ' GB' : m.memory_mb + ' MB'}</span>
					</div>
				{/each}
			</div>
		</div>
	</section>

	<!-- Latency -->
	<section class="panel">
		<h3>Latency (avg)</h3>
		<div class="panel-body">
			{#each [
				{ label: 'STT', ms: $latency.stt_ms },
				{ label: 'Matching', ms: $latency.matching_ms },
				{ label: 'LLM', ms: $latency.llm_ms },
				{ label: 'TTS', ms: $latency.tts_ms },
				{ label: 'Total', ms: $latency.total_ms },
			] as row}
				<div class="latency-row">
					<span class="latency-label">{row.label}</span>
					<div class="bar-track bar-sm">
						<div class="bar-fill" class:accent={row.label === 'Total'} style:width={barWidth(row.ms, 2000)}></div>
					</div>
					<span class="latency-val">{row.ms}ms</span>
				</div>
			{/each}
		</div>
	</section>

	<!-- Processes -->
	<section class="panel">
		<h3>Processes</h3>
		<div class="panel-body">
			{#if $processes.length === 0}
				<p class="empty">No process data available.</p>
			{/if}
			{#each $processes as proc}
				<div class="proc-row">
					<span class="proc-dot" class:running={proc.status === 'running'} class:idle={proc.status === 'idle'}></span>
					<span class="proc-name">{proc.name}</span>
					<span class="proc-status">{proc.status}</span>
					<span class="proc-mem">{proc.memory_mb > 0 ? proc.memory_mb + ' MB' : '—'}</span>
					{#if proc.pid}<span class="proc-pid">PID {proc.pid}</span>{/if}
					{#if proc.status === 'stopped'}
						<button class="btn btn-outline btn-sm">Start</button>
					{/if}
				</div>
			{/each}
		</div>
	</section>

	<!-- Skill Activity -->
	<section class="panel">
		<h3>Skill Activity</h3>
		<div class="panel-body">
			{#if $skillActivity.length === 0}
				<p class="empty">No activity data yet.</p>
			{/if}
			{#each $skillActivity as sa}
				<div class="activity-row">
					<span class="activity-name">{sa.name}</span>
					<div class="bar-track bar-sm">
						<div class="bar-fill accent" style:width="{Math.min(sa.triggers / 60 * 100, 100)}%"></div>
					</div>
					<span class="activity-val">{sa.triggers} today</span>
				</div>
			{/each}
		</div>
	</section>
</div>

<style>
	.system-page { display: flex; flex-direction: column; gap: 16px; }
	.system-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}
	.system-header h2 { font-size: 16px; font-weight: 600; }
	.uptime { font-size: 12px; color: var(--text-secondary); }

	.panel {
		background: var(--bg-surface);
		border: 1px solid var(--border);
		border-radius: var(--radius-lg);
		overflow: hidden;
	}
	.panel h3 {
		font-size: 12px;
		font-weight: 600;
		color: var(--text-secondary);
		text-transform: uppercase;
		letter-spacing: 0.5px;
		padding: 10px 14px;
		border-bottom: 1px solid var(--border);
	}
	.panel-body { padding: 12px 14px; }

	.bar-track {
		height: 8px;
		background: var(--bg-base);
		border-radius: 4px;
		overflow: hidden;
	}
	.bar-sm { height: 6px; }
	.bar-fill {
		height: 100%;
		background: var(--info);
		border-radius: 4px;
		transition: width 0.3s ease;
	}
	.bar-fill.accent { background: var(--accent); }

	.mem-overview { margin-bottom: 12px; }
	.mem-row {
		display: flex;
		justify-content: space-between;
		font-size: 12px;
		margin-bottom: 4px;
	}
	.mem-item {
		display: flex;
		align-items: center;
		gap: 10px;
		font-size: 12px;
		margin: 4px 0 4px 12px;
	}
	.mem-item-name { width: 140px; font-family: var(--font-mono); font-size: 11px; }
	.mem-item .bar-track { flex: 1; }
	.mem-item-val { width: 60px; text-align: right; color: var(--text-secondary); font-size: 11px; }

	.latency-row {
		display: flex;
		align-items: center;
		gap: 10px;
		margin: 4px 0;
		font-size: 12px;
	}
	.latency-label { width: 70px; color: var(--text-secondary); }
	.latency-row .bar-track { flex: 1; }
	.latency-val { width: 60px; text-align: right; font-family: var(--font-mono); font-size: 11px; }

	.proc-row {
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 4px 0;
		font-size: 12px;
	}
	.proc-dot {
		width: 6px;
		height: 6px;
		border-radius: 50%;
		background: var(--text-muted);
		flex-shrink: 0;
	}
	.proc-dot.running { background: var(--success); }
	.proc-dot.idle { background: var(--warning); }
	.proc-name { font-weight: 600; width: 120px; }
	.proc-status { width: 60px; color: var(--text-secondary); }
	.proc-mem { width: 60px; color: var(--text-secondary); font-family: var(--font-mono); font-size: 11px; }
	.proc-pid { color: var(--text-muted); font-size: 11px; }

	.activity-row {
		display: flex;
		align-items: center;
		gap: 10px;
		margin: 4px 0;
		font-size: 12px;
	}
	.activity-name { width: 100px; font-weight: 500; }
	.activity-row .bar-track { flex: 1; }
	.activity-val { width: 70px; text-align: right; color: var(--text-secondary); font-size: 11px; }

	.empty { color: var(--text-muted); font-size: 12px; }
</style>
