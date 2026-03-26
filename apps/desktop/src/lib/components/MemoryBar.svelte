<script lang="ts">
	import { memory, herzenMemoryMb } from '$lib/stores/models.js';

	let mem = $derived($memory);
	let herzenMb = $derived($herzenMemoryMb);
	let budgetGb = $derived(mem.system_total / 1024);
	let usedGb = $derived(herzenMb / 1024);
	let pct = $derived(budgetGb > 0 ? Math.min((usedGb / budgetGb) * 100, 100) : 0);

	let segments = $derived(
		Object.entries(mem.herzen_breakdown).map(([name, mb]) => ({
			name,
			mb,
			pct: budgetGb > 0 ? (mb / 1024 / budgetGb) * 100 : 0,
		}))
	);

	const segmentColors = ['#f5a623', '#60a5fa', '#4ade80', '#a78bfa', '#facc15', '#f87171'];
</script>

<div class="memory-bar">
	<div class="memory-header">
		<span>Memory</span>
		<span class="memory-value">{usedGb.toFixed(1)} / {budgetGb.toFixed(0)} GB</span>
	</div>
	<div class="bar-track">
		{#each segments as seg, i}
			<div
				class="bar-segment"
				style:width="{seg.pct}%"
				style:background={segmentColors[i % segmentColors.length]}
				title="{seg.name}: {(seg.mb / 1024).toFixed(1)} GB"
			></div>
		{/each}
	</div>
	{#if segments.length > 0}
		<div class="memory-legend">
			{#each segments as seg, i}
				<span class="legend-item">
					<span class="legend-dot" style:background={segmentColors[i % segmentColors.length]}></span>
					{seg.name}: {seg.mb >= 1024 ? (seg.mb / 1024).toFixed(1) + ' GB' : seg.mb + ' MB'}
				</span>
			{/each}
		</div>
	{/if}
</div>

<style>
	.memory-bar { width: 100%; }
	.memory-header {
		display: flex;
		justify-content: space-between;
		font-size: 12px;
		margin-bottom: 4px;
	}
	.memory-value { color: var(--text-secondary); }

	.bar-track {
		height: 8px;
		background: var(--bg-base);
		border-radius: 4px;
		display: flex;
		overflow: hidden;
	}
	.bar-segment {
		height: 100%;
		transition: width 0.3s ease;
		min-width: 2px;
	}

	.memory-legend {
		display: flex;
		flex-wrap: wrap;
		gap: 8px;
		margin-top: 4px;
		font-size: 11px;
		color: var(--text-secondary);
	}
	.legend-item { display: flex; align-items: center; gap: 4px; }
	.legend-dot {
		width: 6px;
		height: 6px;
		border-radius: 50%;
		flex-shrink: 0;
	}
</style>
