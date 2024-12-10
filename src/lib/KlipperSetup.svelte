<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';

	async function send() {
		let get_res = await invoke('send_ssh', {
		});
		console.log(get_res)
	}
  
</script>


<div class="card p-4 flex gap-4 flex-col">
  <h2>KlipperSetup</h2>
	<div class="grid grid-cols-4 lg:grid-cols-4 gap-4">
	</div>
	<button class="btn variant-filled" on:click={send}>Send</button>
</div>