<script>
	import favicon from '$lib/assets/favicon.svg';

	export const ssr = false;

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
	<title>Rjombox</title>
</svelte:head>

{@render children()}
