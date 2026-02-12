<script lang="ts">
  import Sidebar from './Sidebar.svelte';
  import ChannelList from './ChannelList.svelte';
  import MainContent from './MainContent.svelte';

  let { identity } = $props<{ identity: any }>();
</script>

<div class="app-shell">
  <Sidebar {identity} />
  <ChannelList />
  <MainContent {identity} />
</div>

<style>
  .app-shell {
    display: flex;
    width: 100vw;
    height: 100vh;
    overflow: hidden;
  }
</style>
