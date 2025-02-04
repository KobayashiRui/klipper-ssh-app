<script lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { ProgressRadial } from '@skeletonlabs/skeleton';

let fluidd_upload = false

let fluidd_file_path: string|null =null


async function send_fluidd_file_select() {
	const file = await open({
		multiple: false,
		directory: false,
	})
	console.log(file)
	fluidd_file_path = file 
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
    <h3 class="h3">WebUI Setup</h3>
	<div>
		<h4 class="h4 mb-1">Fluidd Upload</h4>
		<div>
			<button  type="button" class="btn variant-filled mb-2" on:click={send_fluidd_file_select}>File Select</button>
			{#if fluidd_file_path === null}
				<span>File: Not selected</span>
			{:else}
				<span>File:{fluidd_file_path}</span>
			{/if}
		</div>
		<div class="flex items-center">
			{#if fluidd_upload}
				<button  type="button" class="btn variant-filled disable">
					<span>Fluidd Uploading...</span>
					<ProgressRadial value={undefined} width="w-5" stroke={100} meter="stroke-primary-500" track="stroke-primary-500/30" strokeLinecap="butt" />
				</button>
			{:else}
				<button  type="button" class="btn variant-filled" on:click={upload_fluidd}>Fluidd Upload</button>
			{/if}
		</div>
	</div>
</div>