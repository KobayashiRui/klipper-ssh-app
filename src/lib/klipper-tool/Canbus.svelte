<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';

	let can_uuids:any[] = []

	async function get_can_uuid_list() {
		can_uuids = await invoke('klipper_can_uuid_list')
		console.log(can_uuids)
	}
</script>

<div>
	<button class="btn variant-filled mb-2" on:click={get_can_uuid_list}>Get CAN UUID List</button>
  <div class="table-container">
  	<table class="table table-hover">
  		<thead>
  			<tr>
  				<th>Device</th>
  				<th>UUID</th>
  			</tr>
  		</thead>
  		<tbody>
        {#if can_uuids.length > 0}
  			  {#each can_uuids as can_dev}
			      {#each  can_dev.can_uuids as can_uuid}
  			  	  <tr>
  			  	  	<td>{can_dev.device_name}</td>
  			  	  	<td>{can_uuid}</td>
  			  	  </tr>
			      {/each}
          {/each}
        {:else}
          <tr>
            <td colspan="2">Not found...</td>
          </tr>
        {/if}
  		</tbody>
  		<tfoot>
  			<tr>
  				<th>Get UUIDs</th>
  				<td>{can_uuids.length} nodes</td>
  			</tr>
  		</tfoot>
  	</table>
  </div>
</div>
