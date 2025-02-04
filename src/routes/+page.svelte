<script lang="ts">
	import SshClient from '$lib/SshClient.svelte';
	import KlipperSetup from '$lib/KlipperSetup.svelte';
	//import DfuSetup from '$lib/DFUSetup.svelte';
	import WebUISetup from '$lib/WebUISetup.svelte';
	import KlipperTool from '$lib/KlipperTool.svelte';
</script>

<div class="container h-full mx-auto py-4 w-full grid grid-cols-1 md:grid-cols-2 gap-y-4 gap-x-4 justify-center">
		<h2 class="h2 mt-2">Klipper SSH</h2>
		<SshClient />
		<KlipperSetup />
		<WebUISetup />
		<KlipperTool />
</div>