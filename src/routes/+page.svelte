<script lang="ts">
	import Greet from '$lib/Greet.svelte';
	import SshClient from '$lib/SshClient.svelte';
	import KlipperSetup from '$lib/KlipperSetup.svelte';
	import DfuSetup from '$lib/DFUSetup.svelte';
	import WebUISetup from '$lib/WebUISetup.svelte';
</script>

<div class="container h-full mx-auto flex justify-center items-center">
	<div class="space-y-5">
		<h2 class="h2">Klipper SSH</h2>
		<SshClient />
		<KlipperSetup />
		<WebUISetup />
		<DfuSetup />
		<Greet />
	</div>
</div>