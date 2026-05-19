<template>
  <div class="h-8 bg-gray-50 border-t border-gray-200 flex items-center justify-between px-4 text-sm">
    <div class="flex items-center gap-4 text-gray-600">
      <span>已选: <strong class="text-blue-600">{{ selectedCount }}</strong></span>
      <span>总数: <strong>{{ totalCount }}</strong></span>
      <span>重复: <strong class="text-red-500">{{ duplicateCount }}</strong></span>
    </div>
    <div class="text-gray-400">
      {{ exportPath || '未选择导出路径' }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { usePhotoStore } from '../stores/photoStore';

const store = usePhotoStore();
const { selectedCount, totalCount, duplicateCount, exportPath } = store;
</script>