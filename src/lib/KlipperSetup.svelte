<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { open } from '@tauri-apps/plugin-dialog';
	import { ProgressRadial } from '@skeletonlabs/skeleton';

	let can_devices:string[] = []
	let can_uuids:any[] = []

	let fw_upload = false
	let fluidd_upload = false

	let klipper_file_path: string|null = null
	let fluidd_file_path: string|null =null

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

	async function send_fw_file_select() {
		const file = await open({
			multiple: false,
			directory: false,
		})
		console.log(file)
		klipper_file_path = file 
	}

	async function send_fluidd_file_select() {
		const file = await open({
			multiple: false,
			directory: false,
		})
		console.log(file)
		fluidd_file_path = file 
	}

	async function upload_fw() {
		fw_upload = true
		if(klipper_file_path === null){
			window.alert("ファイルを選択してください")
		}else{
			const res = await invoke('klipper_send_fw', {localPath:klipper_file_path})
			console.log(res)
		}
		fw_upload = false
	}

	async function upload_fluidd() {
		fluidd_upload = true
		if(fluidd_file_path === null){
			window.alert("ファイルを選択してください")
		}else{
			const res = await invoke('klipper_send_fluidd', {localPath:fluidd_file_path})
			console.log(res)
		}
		fluidd_upload = false
	}

</script>


<div class="card p-4 flex gap-4 flex-col">
  <h3 class="h3">Klipper Setup</h3>
	<div>
		<h4 class="h4 mb-1">Klipper Firmware Upload</h4>
		<div>
			<button  type="button" class="btn variant-filled mb-2" on:click={send_fw_file_select}>File Select</button>
			{#if klipper_file_path === null}
				<span>File: Not selected</span>
			{:else}
				<span>File:{klipper_file_path}</span>
			{/if}
		</div>
		<div class="flex items-center">
			{#if fw_upload}
				<button  type="button" class="btn variant-filled disable">
					<span>Firmware Uploading...</span>
					<ProgressRadial value={undefined} width="w-5" stroke={100} meter="stroke-primary-500" track="stroke-primary-500/30" strokeLinecap="butt" />
				</button>
			{:else}
				<button  type="button" class="btn variant-filled" on:click={upload_fw}>Firmware Upload</button>
			{/if}
		</div>
	</div>
	<div>
		<h4 class="h4 mb-1">Fluidd Upload</h4>
		<div>
			<button  type="button" class="btn variant-filled mb-2" on:click={send_fluidd_file_select}>File Select</button>
			{#if klipper_file_path === null}
				<span>File: Not selected</span>
			{:else}
				<span>File:{klipper_file_path}</span>
			{/if}
		</div>
		<div class="flex items-center">
			{#if fw_upload}
				<button  type="button" class="btn variant-filled disable">
					<span>Fluidd Uploading...</span>
					<ProgressRadial value={undefined} width="w-5" stroke={100} meter="stroke-primary-500" track="stroke-primary-500/30" strokeLinecap="butt" />
				</button>
			{:else}
				<button  type="button" class="btn variant-filled" on:click={upload_fluidd}>Fluidd Upload</button>
			{/if}
		</div>
	</div>
	<div>
		<h4 class="h4 mb-1">CAN UUID</h4>
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