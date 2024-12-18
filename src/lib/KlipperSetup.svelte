<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';

	let can_devices:string[] = []
	let can_uuids:any[] = []

	async function send() {
		let get_res = await invoke('send_ssh', {
		});
		console.log(get_res)
	}

	async function get_can_interface() {
		can_devices = await invoke('klipper_can_interface')
	}

	async function get_can_uuid_list() {
		can_uuids = await invoke('klipper_can_uuid_list')
		console.log(can_uuids)
	}
  
</script>


<div class="card p-4 flex gap-4 flex-col">
  <h2>KlipperSetup</h2>
	<div class="grid grid-cols-4 lg:grid-cols-4 gap-4">
	</div>
	<button class="btn variant-filled" on:click={send}>Send</button>
	<h3>CAN UUID</h3>
	<ul>
		{#each can_uuids as can_dev}
			{#each  can_dev.can_uuids as can_uuid}
				<li>{can_dev.device_name}: {can_uuid}</li>
			{/each}
		{/each}
	</ul>
	<button class="btn variant-filled" on:click={get_can_interface}>Get CAN Interface</button>
	<button class="btn variant-filled" on:click={get_can_uuid_list}>Get CAN UUID List</button>
</div>