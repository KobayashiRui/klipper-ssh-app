<script lang="ts">
  import { TabGroup, Tab, TabAnchor } from '@skeletonlabs/skeleton';
  import Canbus from './klipper-tool/Canbus.svelte';
  let tabSet: number = 0;

</script>


<div class="card md:col-span-2 p-4 flex gap-4 flex-col">
  <h3 class="h3">Klipper Tool</h3>
  <TabGroup>
  	<Tab bind:group={tabSet} name="tab1" value={0}>CAN bus</Tab>
  	<Tab bind:group={tabSet} name="tab2" value={1}>...</Tab>
  	<Tab bind:group={tabSet} name="tab3" value={2}>...</Tab>
  	<!-- Tab Panels --->
  	<svelte:fragment slot="panel">
  		{#if tabSet === 0}
        <Canbus></Canbus>
  		{:else if tabSet === 1}
        todo...
  		{:else if tabSet === 2}
        todo...
  		{/if}
  	</svelte:fragment>
  </TabGroup>


</div>