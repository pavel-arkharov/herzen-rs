<script lang="ts">
	import { page } from '$app/stores';
	import Orb from './Orb.svelte';

	const nav = [
		{ path: '/chat', icon: '💬', label: 'Chat' },
		{ path: '/models', icon: '🧠', label: 'Models' },
		{ path: '/workshop', icon: '🔧', label: 'Workshop' },
		{ path: '/system', icon: '📊', label: 'System' },
	];

	let currentPath = $derived($page.url.pathname);
</script>

<aside class="sidebar">
	<div class="sidebar-orb">
		<Orb />
	</div>

	<nav class="sidebar-nav">
		{#each nav as item}
			<a
				href={item.path}
				class="nav-item"
				class:active={currentPath.startsWith(item.path)}
				data-sveltekit-preload-data
			>
				<span class="nav-icon">{item.icon}</span>
				<span class="nav-label">{item.label}</span>
			</a>
		{/each}
	</nav>

	<div class="sidebar-footer">
		<button class="nav-item" title="Settings">
			<span class="nav-icon">⚙️</span>
		</button>
	</div>
</aside>

<style>
	.sidebar {
		width: var(--sidebar-width);
		height: 100%;
		display: flex;
		flex-direction: column;
		background: var(--bg-surface);
		border-right: 1px solid var(--border);
		flex-shrink: 0;
	}

	.sidebar-orb {
		padding: 12px 0;
		display: flex;
		justify-content: center;
		border-bottom: 1px solid var(--border);
	}

	.sidebar-nav {
		flex: 1;
		display: flex;
		flex-direction: column;
		padding: 8px 0;
		gap: 2px;
	}

	.nav-item {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 2px;
		padding: 8px 4px;
		text-decoration: none;
		color: var(--text-secondary);
		border-radius: var(--radius);
		margin: 0 4px;
		transition: background 0.15s, color 0.15s;
	}
	.nav-item:hover {
		background: var(--bg-elevated);
		color: var(--text-primary);
	}
	.nav-item.active {
		color: var(--accent);
		background: rgba(245, 166, 35, 0.1);
	}

	.nav-icon { font-size: 18px; line-height: 1; }
	.nav-label { font-size: 9px; text-transform: uppercase; letter-spacing: 0.3px; }

	.sidebar-footer {
		padding: 8px 0;
		border-top: 1px solid var(--border);
		display: flex;
		justify-content: center;
	}
</style>
