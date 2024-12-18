<script lang="ts">
	import Greet from '$lib/Greet.svelte';
	import SshClient from '$lib/SshClient.svelte';
	import KlipperSetup from '$lib/KlipperSetup.svelte';
	import DfuSetup from '$lib/DFUSetup.svelte';
</script>

<div class="container h-full mx-auto flex justify-center items-center">
	<div class="space-y-5">
		<h1 class="h1">Klipper SSH</h1>
		<SshClient />
		<KlipperSetup />
		<DfuSetup />
		<Greet />
	</div>
</div>