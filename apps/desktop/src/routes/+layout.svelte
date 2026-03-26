<script lang="ts">
	import '../app.css';
	import Sidebar from '$lib/components/Sidebar.svelte';
	import { connect } from '$lib/stores/connection.js';
	import { onMount } from 'svelte';
	import type { Snippet } from 'svelte';

	let { children }: { children: Snippet } = $props();

	onMount(() => {
		connect();
	});
</script>

<div class="app-shell">
	<Sidebar />
	<main class="content">
		{@render children()}
	</main>
</div>

<style>
	.app-shell {
		display: flex;
		height: 100vh;
		width: 100vw;
		overflow: hidden;
	}
	.content {
		flex: 1;
		overflow-y: auto;
		padding: 20px 24px;
	}
</style>
