<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { ask } from '@tauri-apps/plugin-dialog';

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

	async function send_file_select() {
		const answer = await ask('This action cannot be reverted. Are you sure?', {
		  title: 'Tauri',
		  kind: 'warning',
		});
		
		console.log(answer);
	}
</script>


<div class="card p-4 flex gap-4 flex-col">
  <h3 class="h3">Klipper Setup</h3>
	<button class="btn variant-filled" on:click={send}>Send</button>
	<div>
		<h4 class="h4">KlipperFileUpload</h4>
		<button class="btn" on:click={send_file_select}>File Select</button>

	</div>
	<div>
		<h4 class="h4">CAN UUID</h4>
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
</div>