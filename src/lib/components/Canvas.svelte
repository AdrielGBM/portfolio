<script>
	import { SvelteFlow, Background, BackgroundVariant } from '@xyflow/svelte';
	import '@xyflow/svelte/dist/style.css';

	let nodes = $state.raw([
		{ id: '1', position: { x: 0, y: 0 }, data: { label: '1' }, type: 'input' },
		{ id: '2', position: { x: 0, y: 100 }, data: { label: '2' }, type: 'output' }
	]);

	let edges = $state.raw([{ id: 'e1-2', source: '1', target: '2' }]);
</script>

<div class="canvas-wrapper">
	<SvelteFlow bind:nodes bind:edges fitView colorMode="system">
		<Background variant={BackgroundVariant.Dots} gap={12} size={1} />
	</SvelteFlow>
</div>

<style>
	.canvas-wrapper {
		flex: 1;
		overflow: hidden;
		mask-image: linear-gradient(to right, transparent 0%, black 8%);

		:global(.svelte-flow__edges svg) {
			width: unset;
			max-width: unset;
		}
	}
</style>
