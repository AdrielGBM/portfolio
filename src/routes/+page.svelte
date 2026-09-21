<script>
	import Narrative from '$lib/components/Narrative.svelte';
	import Canvas from '$lib/components/Canvas.svelte';
</script>

<main class="layout">
	<Narrative />
	<Canvas />
</main>

<style>
	.layout {
		display: flex;
		width: 100dvw;
		height: 100dvh;
		background: var(--color-neutral-950);
		color: var(--color-neutral-50);
	}
</style>
