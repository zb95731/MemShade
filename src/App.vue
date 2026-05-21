<template>
  <div class="h-screen flex flex-col bg-[#0f172a] text-[#f8fafc] overflow-hidden">
    <div class="absolute inset-0 bg-gradient-to-br from-[#0f172a] via-[#1e293b] to-[#0f172a] pointer-events-none" />
    <div class="absolute inset-0 opacity-5 pointer-events-none animate-grid-pattern" style="background-image: radial-gradient(circle at 1px 1px, #6366f1 1px, transparent 0); background-size: 50px 50px;" />
    
    <Toolbar class="relative z-10" />
    
    <div class="flex-1 flex overflow-hidden relative z-10">
      <SourcePanel />
      <div class="flex-1 relative overflow-hidden">
        <PhotoGrid />
      </div>
    </div>
    
    <StatusBar class="relative z-10" />
  </div>
</template>

<script setup lang="ts">
import Toolbar from './components/Toolbar.vue';
import SourcePanel from './components/SourcePanel.vue';
import PhotoGrid from './components/PhotoGrid.vue';
import StatusBar from './components/StatusBar.vue';
</script>
