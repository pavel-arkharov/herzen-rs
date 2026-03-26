<script lang="ts">
	import { skills, selectedSkill, testResults, refreshSkills, testInput } from '$lib/stores/skills.js';
	import { onMount } from 'svelte';

	onMount(() => { refreshSkills(); });

	let selected = $derived($skills.find((s) => s.name === $selectedSkill) ?? $skills[0] ?? null);
	let testText = $state('');
	let results = $derived($testResults);

	function handleTest() {
		const text = testText.trim();
		if (text) testInput(text);
	}

	function handleTestKey(e: KeyboardEvent) {
		if (e.key === 'Enter') { e.preventDefault(); handleTest(); }
	}

	function selectSkill(name: string) {
		selectedSkill.set(name);
	}
</script>

<div class="workshop-page">
	<div class="workshop-panels">
		<!-- Skill List -->
		<div class="skill-list-panel">
			<h3>Skills</h3>
			<div class="skill-list">
				{#each $skills as skill (skill.name)}
					<button
						class="skill-item"
						class:active={selected?.name === skill.name}
						onclick={() => selectSkill(skill.name)}
					>
						<span class="skill-dot" class:enabled={skill.enabled}></span>
						<span>{skill.name}</span>
					</button>
				{/each}
				{#if $skills.length === 0}
					<p class="empty">No skills loaded. Add .toml files to skills directory.</p>
				{/if}
			</div>
			<button class="btn btn-outline btn-sm" style="margin-top: 8px; width: 100%;">+ New Skill</button>
		</div>

		<!-- Skill Editor -->
		<div class="skill-editor-panel">
			{#if selected}
				<div class="editor-header">
					<h2>Skill: {selected.name}</h2>
					<span class="badge" class:badge-success={selected.enabled} class:badge-warning={!selected.enabled}>
						{selected.enabled ? 'Enabled' : 'Disabled'}
					</span>
				</div>

				<div class="editor-fields">
					<label class="field">
						<span>Name</span>
						<input type="text" value={selected.name} readonly />
					</label>
					<label class="field">
						<span>Description</span>
						<input type="text" value={selected.description} />
					</label>
					<div class="field-row">
						<label class="field">
							<span>Priority</span>
							<input type="number" value={selected.priority} style="width: 80px;" />
						</label>
						<label class="field-check">
							<input type="checkbox" checked={selected.enabled} />
							<span>Enabled</span>
						</label>
					</div>
				</div>

				<details class="editor-section" open>
					<summary>Intent Slots</summary>
					<div class="slots">
						{#each selected.slots as slot}
							<div class="slot-card">
								<div class="slot-header">
									<span class="slot-name">{slot.name}</span>
									<span class="badge" class:badge-error={slot.required} class:badge-info={!slot.required}>
										{slot.required ? 'required' : 'optional'}
									</span>
								</div>
								{#each Object.entries(slot.values) as [canonical, langs]}
									<div class="slot-value">
										<strong>{canonical}:</strong>
										{#each Object.entries(langs) as [lang, keywords]}
											<span class="slot-lang">{lang}: {keywords.join(', ')}</span>
										{/each}
									</div>
								{/each}
							</div>
						{/each}
						<button class="btn btn-outline btn-sm">+ Add Slot</button>
					</div>
				</details>

				<details class="editor-section">
					<summary>Semantic Examples</summary>
					<div class="examples">
						{#each selected.semantic_examples as ex}
							<div class="example-row">
								<span>{ex}</span>
								<button class="btn btn-ghost btn-sm">×</button>
							</div>
						{/each}
						<div class="example-row">
							<input type="text" placeholder="Add example..." style="flex: 1;" />
							<button class="btn btn-outline btn-sm">+</button>
						</div>
						<div class="threshold">
							Threshold: <code>{selected.semantic_threshold}</code>
						</div>
					</div>
				</details>

				<details class="editor-section">
					<summary>Confidence Gates</summary>
					<div class="gates">
						<label class="slider-row">
							<span>Auto-execute</span>
							<input type="range" min="0" max="1" step="0.05" value={selected.confidence_gates.auto_execute} />
							<span class="slider-value">{selected.confidence_gates.auto_execute.toFixed(2)}</span>
						</label>
						<label class="slider-row">
							<span>Confirm above</span>
							<input type="range" min="0" max="1" step="0.05" value={selected.confidence_gates.confirm_above} />
							<span class="slider-value">{selected.confidence_gates.confirm_above.toFixed(2)}</span>
						</label>
						<label class="field-check">
							<input type="checkbox" checked={selected.confidence_gates.destructive} />
							<span>Destructive</span>
						</label>
					</div>
				</details>

				<details class="editor-section">
					<summary>Response</summary>
					<div class="response-fields">
						<label class="field">
							<span>Template</span>
							<input type="text" value={selected.response_template} />
						</label>
						<label class="field">
							<span>Confirm</span>
							<input type="text" value={selected.confirm_template} />
						</label>
					</div>
				</details>
			{:else}
				<div class="editor-empty">
					<p>Select a skill to edit, or create a new one.</p>
				</div>
			{/if}
		</div>
	</div>

	<!-- Test Bench -->
	<div class="test-bench">
		<h3>Test Bench</h3>
		<div class="test-input-row">
			<input
				type="text"
				class="test-input"
				placeholder="Type test input in any language..."
				bind:value={testText}
				onkeydown={handleTestKey}
			/>
			<button class="btn btn-primary btn-sm" onclick={handleTest}>Test</button>
		</div>
		{#if results.length > 0}
			<div class="test-results">
				{#each results as r}
					<div class="result-row" class:matched={r.decision !== 'reject'}>
						<span class="result-icon">{r.decision === 'execute' ? '✅' : r.decision === 'confirm' ? '⚠️' : '○'}</span>
						<span class="result-skill">{r.skill}</span>
						<span class="result-conf">{r.confidence.toFixed(2)}</span>
						<span class="result-method">{r.method}</span>
						{#if Object.keys(r.slots).length > 0}
							<span class="result-slots">
								{#each Object.entries(r.slots) as [k, v]}
									{k}={v}
								{/each}
							</span>
						{/if}
						<span class="result-decision badge" class:badge-success={r.decision === 'execute'} class:badge-warning={r.decision === 'confirm'} class:badge-error={r.decision === 'reject'}>
							{r.decision.toUpperCase()}
						</span>
					</div>
				{/each}
			</div>
		{/if}
	</div>
</div>

<style>
	.workshop-page {
		display: flex;
		flex-direction: column;
		height: 100%;
		gap: 0;
	}

	.workshop-panels {
		display: flex;
		flex: 1;
		gap: 16px;
		overflow: hidden;
	}

	/* Skill List */
	.skill-list-panel {
		width: 180px;
		flex-shrink: 0;
		display: flex;
		flex-direction: column;
	}
	.skill-list-panel h3 {
		font-size: 13px;
		font-weight: 600;
		color: var(--text-secondary);
		text-transform: uppercase;
		letter-spacing: 0.5px;
		margin-bottom: 8px;
	}
	.skill-list {
		flex: 1;
		overflow-y: auto;
	}
	.skill-item {
		display: flex;
		align-items: center;
		gap: 8px;
		width: 100%;
		padding: 6px 10px;
		border-radius: var(--radius);
		text-align: left;
		color: var(--text-primary);
		transition: background 0.15s;
	}
	.skill-item:hover { background: var(--bg-elevated); }
	.skill-item.active { background: rgba(245, 166, 35, 0.1); color: var(--accent); }
	.skill-dot {
		width: 6px;
		height: 6px;
		border-radius: 50%;
		background: var(--text-muted);
		flex-shrink: 0;
	}
	.skill-dot.enabled { background: var(--success); }

	/* Skill Editor */
	.skill-editor-panel {
		flex: 1;
		overflow-y: auto;
		padding-right: 8px;
	}
	.editor-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 16px;
	}
	.editor-header h2 { font-size: 16px; font-weight: 600; }

	.editor-fields {
		display: flex;
		flex-direction: column;
		gap: 8px;
		margin-bottom: 16px;
	}
	.field {
		display: flex;
		flex-direction: column;
		gap: 3px;
	}
	.field span {
		font-size: 11px;
		color: var(--text-secondary);
		text-transform: uppercase;
		letter-spacing: 0.3px;
	}
	.field-row {
		display: flex;
		gap: 16px;
		align-items: flex-end;
	}
	.field-check {
		display: flex;
		align-items: center;
		gap: 6px;
		font-size: 12px;
	}

	.editor-section {
		border: 1px solid var(--border);
		border-radius: var(--radius);
		margin-bottom: 10px;
	}
	.editor-section summary {
		padding: 8px 12px;
		font-size: 12px;
		font-weight: 600;
		color: var(--text-secondary);
		cursor: pointer;
	}
	.editor-section summary:hover { color: var(--text-primary); }
	.editor-section > div {
		padding: 10px 12px;
		border-top: 1px solid var(--border);
	}

	.slot-card {
		background: var(--bg-base);
		border-radius: var(--radius);
		padding: 8px 10px;
		margin-bottom: 8px;
	}
	.slot-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 4px;
	}
	.slot-name { font-weight: 600; font-family: var(--font-mono); font-size: 13px; }
	.slot-value {
		font-size: 12px;
		color: var(--text-secondary);
		margin-left: 8px;
		line-height: 1.6;
	}
	.slot-lang {
		display: inline;
		margin-left: 4px;
		font-family: var(--font-mono);
		font-size: 11px;
	}

	.example-row {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 8px;
		padding: 3px 0;
		font-size: 12px;
	}
	.threshold {
		margin-top: 8px;
		font-size: 11px;
		color: var(--text-secondary);
	}
	.threshold code { color: var(--accent); }

	.gates { display: flex; flex-direction: column; gap: 8px; }
	.slider-row {
		display: flex;
		align-items: center;
		gap: 10px;
		font-size: 12px;
		color: var(--text-secondary);
	}
	.slider-row span:first-child { width: 100px; flex-shrink: 0; }
	.slider-row input[type="range"] { flex: 1; accent-color: var(--accent); }
	.slider-value { width: 40px; text-align: right; font-family: var(--font-mono); }

	.response-fields { display: flex; flex-direction: column; gap: 8px; }
	.editor-empty {
		display: flex;
		align-items: center;
		justify-content: center;
		height: 200px;
		color: var(--text-muted);
	}

	/* Test Bench */
	.test-bench {
		flex-shrink: 0;
		border-top: 1px solid var(--border);
		padding-top: 12px;
		margin-top: 12px;
	}
	.test-bench h3 {
		font-size: 13px;
		font-weight: 600;
		color: var(--text-secondary);
		text-transform: uppercase;
		letter-spacing: 0.5px;
		margin-bottom: 8px;
	}
	.test-input-row {
		display: flex;
		gap: 8px;
		margin-bottom: 10px;
	}
	.test-input {
		flex: 1;
		padding: 6px 12px;
	}

	.test-results { display: flex; flex-direction: column; gap: 4px; }
	.result-row {
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 4px 8px;
		font-size: 12px;
		border-radius: var(--radius);
		background: var(--bg-surface);
	}
	.result-row.matched { background: rgba(245, 166, 35, 0.05); }
	.result-icon { font-size: 14px; }
	.result-skill { font-weight: 600; width: 100px; }
	.result-conf { font-family: var(--font-mono); width: 40px; }
	.result-method { color: var(--text-secondary); width: 70px; }
	.result-slots {
		flex: 1;
		font-family: var(--font-mono);
		font-size: 11px;
		color: var(--text-secondary);
	}

	.empty { color: var(--text-muted); font-size: 12px; }
</style>
