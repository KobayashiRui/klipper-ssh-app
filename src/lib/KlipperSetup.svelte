<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { open } from '@tauri-apps/plugin-dialog';
	import { ProgressRadial } from '@skeletonlabs/skeleton';

	let fw_upload = false
	let klipper_file_path: string|null = null

	async function send_fw_file_select() {
		const file = await open({
			multiple: false,
			directory: false,
		})
		console.log(file)
		klipper_file_path = file 
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
</div>