<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';

	async function get_dfu_devices() {
		let get_res = await invoke('get_dfu_devices', {
		});
		console.log(get_res)
	}
  
</script>

<div class="card p-4 flex gap-4 flex-col">
  <h2>DFUSetup</h2>
	<div class="grid grid-cols-4 lg:grid-cols-4 gap-4">
	</div>
	<button class="btn variant-filled" on:click={get_dfu_devices}>Get DFU devices</button>
</div>