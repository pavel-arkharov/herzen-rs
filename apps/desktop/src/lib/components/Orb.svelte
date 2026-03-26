<script lang="ts">
	import { orbState, audioLevel } from '$lib/stores/orb.js';

	let state = $derived($orbState);
	let level = $derived($audioLevel);

	const stateColors: Record<string, string> = {
		idle: 'var(--accent)',
		listening: '#60a5fa',
		processing: 'var(--accent)',
		speaking: '#4ade80',
		error: '#ef4444',
		thinking: '#a78bfa',
	};

	let color = $derived(stateColors[state] ?? 'var(--accent)');
	let scale = $derived(state === 'listening' ? 1 + level * 0.15 : 1);
</script>

<div
	class="orb-container"
	class:idle={state === 'idle'}
	class:listening={state === 'listening'}
	class:processing={state === 'processing'}
	class:speaking={state === 'speaking'}
	class:error={state === 'error'}
	class:thinking={state === 'thinking'}
>
	<div
		class="orb"
		style:--orb-color={color}
		style:--orb-scale={scale}
	>
		<div class="orb-core"></div>
		<div class="orb-glow"></div>
		{#if state === 'speaking'}
			<div class="orb-ring ring-1"></div>
			<div class="orb-ring ring-2"></div>
			<div class="orb-ring ring-3"></div>
		{/if}
	</div>
	{#if state !== 'idle'}
		<span class="orb-label">
			{state === 'listening' ? 'Listening...' :
			 state === 'processing' ? 'Processing...' :
			 state === 'speaking' ? 'Speaking...' :
			 state === 'thinking' ? 'Thinking...' :
			 state === 'error' ? 'Error' : ''}
		</span>
	{/if}
</div>

<style>
	.orb-container {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 8px;
	}

	.orb {
		position: relative;
		width: 48px;
		height: 48px;
		transform: scale(var(--orb-scale, 1));
		transition: transform 0.1s ease-out;
	}

	.orb-core {
		position: absolute;
		inset: 0;
		border-radius: 50%;
		background: radial-gradient(
			circle at 35% 35%,
			color-mix(in srgb, var(--orb-color) 80%, white),
			var(--orb-color),
			color-mix(in srgb, var(--orb-color) 60%, black)
		);
		transition: background 0.4s ease;
	}

	.orb-glow {
		position: absolute;
		inset: -8px;
		border-radius: 50%;
		background: radial-gradient(circle, var(--orb-color), transparent 70%);
		opacity: 0.3;
		transition: opacity 0.4s ease;
	}

	.idle .orb {
		animation: breathe 3s ease-in-out infinite;
	}
	.idle .orb-glow { opacity: 0.2; }

	.listening .orb-glow { opacity: 0.5; }

	.processing .orb-core {
		animation: spin-glow 2s linear infinite;
	}

	.thinking .orb-core {
		animation: think-pulse 1.5s ease-in-out infinite;
	}

	.error .orb {
		animation: shake 0.3s ease-in-out;
	}

	.orb-ring {
		position: absolute;
		inset: -4px;
		border-radius: 50%;
		border: 1.5px solid var(--orb-color);
		opacity: 0;
		animation: ring-expand 1.5s ease-out infinite;
	}
	.ring-2 { animation-delay: 0.5s; }
	.ring-3 { animation-delay: 1s; }

	.orb-label {
		font-size: 10px;
		color: var(--text-secondary);
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	@keyframes breathe {
		0%, 100% { transform: scale(0.98); }
		50% { transform: scale(1.02); }
	}

	@keyframes spin-glow {
		from { filter: hue-rotate(0deg); }
		to { filter: hue-rotate(360deg); }
	}

	@keyframes think-pulse {
		0%, 100% { opacity: 0.8; transform: scale(1); }
		50% { opacity: 1; transform: scale(1.05); }
	}

	@keyframes shake {
		0%, 100% { transform: translateX(0); }
		25% { transform: translateX(-4px); }
		75% { transform: translateX(4px); }
	}

	@keyframes ring-expand {
		0% { transform: scale(1); opacity: 0.6; }
		100% { transform: scale(2.5); opacity: 0; }
	}
</style>
