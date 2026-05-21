<template>
  <div class="h-10 bg-[rgba(15,23,42,0.98)] backdrop-blur-xl border-t border-[rgba(71,85,105,0.5)] flex items-center justify-between px-6 text-sm">
    <div class="flex items-center gap-6">
      <div class="flex items-center gap-2">
        <div class="w-1.5 h-1.5 rounded-full bg-gradient-to-r from-indigo-400 to-purple-400"></div>
        <span class="text-slate-400">已选:</span>
        <span class="font-semibold text-indigo-400">{{ selectedCount }}</span>
      </div>
      <div class="flex items-center gap-2">
        <div class="w-1.5 h-1.5 rounded-full bg-gradient-to-r from-emerald-400 to-cyan-400"></div>
        <span class="text-slate-400">总数:</span>
        <span class="font-semibold text-slate-200">{{ totalCount }}</span>
      </div>
      <div class="flex items-center gap-2">
        <div class="w-1.5 h-1.5 rounded-full bg-gradient-to-r from-red-400 to-orange-400"></div>
        <span class="text-slate-400">重复:</span>
        <span class="font-semibold text-red-400">{{ duplicateCount }}</span>
      </div>
    </div>
    <div class="flex items-center gap-2">
      <div class="w-1.5 h-1.5 rounded-full bg-gradient-to-r from-amber-400 to-yellow-400"></div>
      <span class="text-slate-500 truncate max-w-md">{{ exportPath || '未选择导出路径' }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { usePhotoStore } from '../stores/photoStore';

const store = usePhotoStore();
const { selectedCount, totalCount, duplicateCount, exportPath } = store;
</script>