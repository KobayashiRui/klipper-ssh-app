<script lang="ts">
	import {connected} from '$lib/store';
	import { invoke } from '@tauri-apps/api/core';

	let host = ''
	let username = ''
	let password = ''
	let port = '22'

	let connected_value: boolean;
	connected.subscribe((value) => {
		console.log("connected:", value)
		connected_value = value;
	})

	async function connect() {
		let get_res = await invoke('connect_ssh', {
			host, username, password, port: parseInt(port)
		});
		console.log(get_res)
		if(get_res === "Connected"){
			connected.update(()=>true);
		}
	}

	async function disconnect() {
		let get_res = await invoke('disconnect_ssh')
		if(get_res === "Disconnected" || get_res === "NotConnected"){
			connected.update(()=>false)
		}
	}

	async function send() {
		let get_res = await invoke('send_ssh', {
		});
		console.log(get_res)
	}

</script>

<div class="card md:col-span-2 p-4 flex gap-4 flex-col">
	<div class="flex gap-4">
		<h3 class="h3">SSH Connect</h3>
		{#if connected_value}
			<span class="chip variant-filled-success">Connected</span>
		{:else}
			<span class="chip variant-filled-error">Not connected</span>
		{/if}
	</div>
	<div class="grid grid-cols-4 lg:grid-cols-4 gap-4">
		<label class="label">
			<span>host name</span>
			<input id="host-input" placeholder="Enter a host name..." bind:value={host} class="input" autocomplete="off"/>
		</label>
		<label class="label">
			<span>user name</span>
			<input id="username-input" placeholder="Enter a user name..." bind:value={username} class="input" autocomplete='off'/>
		</label>
		<label class="label">
			<span>password</span>
			<input id="password-input" type="password" placeholder="Enter a password..." bind:value={password} class="input" autocomplete="off"/>
		</label>
		<label class="port">
			<span>port</span>
			<input id="port-input" placeholder="Enter a port..." bind:value={port} class="input number" autocomplete="off"/>
		</label>
	</div>	

	{#if connected_value}
		<button class="btn variant-filled-surface" on:click={disconnect}>Disconnect</button>
	{:else}
		<button class="btn variant-filled" on:click={connect}>Connect</button>
	{/if}
</div>