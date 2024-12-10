<script lang="ts">
	import {connected} from '$lib/store';
	import { invoke } from '@tauri-apps/api/core';

	let host = ''
	let username = ''
	let password = ''
	let port = '22'

	let connected_value: boolean;
	connected.subscribe((value) => {
		console.log("connected")
		console.log(value)
		connected_value = value;
	})

	async function connect() {
		let get_res = await invoke('connect_ssh', {
			host, username, password, port: parseInt(port)
		});
		console.log(get_res)
		if(get_res === "Connect SSH"){
			connected.update(()=>true);
		}
	}

	async function send() {
		let get_res = await invoke('send_ssh', {
		});
		console.log(get_res)
	}

</script>

<div class="card p-4 flex gap-4 flex-col">
	<div class="grid grid-cols-4 lg:grid-cols-4 gap-4">
		<label class="label">
			<span>host name</span>
			<input id="host-input" placeholder="Enter a name..." bind:value={host} class="input" autocomplete="off"/>
		</label>
		<label class="label">
			<span>user name</span>
			<input id="username-input" placeholder="Enter a name..." bind:value={username} class="input" autocomplete='off'/>
		</label>
		<label class="label">
			<span>password</span>
			<input id="password-input" placeholder="Enter a name..." bind:value={password} class="input" autocomplete="off"/>
		</label>
		<label class="port">
			<span>port</span>
			<input id="password-input" placeholder="Enter a name..." bind:value={port} class="input number" autocomplete="off"/>
		</label>
	</div>
	{#if connected_value}
		<button class="btn variant-filled-success" on:click={connect} disabled>Connected!</button>
	{:else}
		<button class="btn variant-filled" on:click={connect}>Connect</button>
	{/if}
</div>