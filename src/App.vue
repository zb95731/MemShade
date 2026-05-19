<template>
  <div class="h-screen flex flex-col bg-white">
    <Toolbar />
    <div class="flex-1 flex overflow-hidden">
      <SourcePanel />
      <PhotoGrid />
    </div>
    <StatusBar />
  </div>
</template>

<script setup lang="ts">
import Toolbar from './components/Toolbar.vue';
import SourcePanel from './components/SourcePanel.vue';
import PhotoGrid from './components/PhotoGrid.vue';
import StatusBar from './components/StatusBar.vue';
</script>