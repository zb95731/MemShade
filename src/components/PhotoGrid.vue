<template>
  <div class="flex-1 overflow-auto p-4">
    <div v-if="isScanning" class="flex items-center justify-center h-64">
      <div class="flex items-center gap-2">
        <Loader2 class="w-6 h-6 animate-spin" />
        <span class="text-gray-500">正在扫描...</span>
      </div>
    </div>
    
    <div v-else-if="photos.length === 0" class="flex items-center justify-center h-64">
      <div class="text-center text-gray-400">
        <ImageIcon class="w-12 h-12 mx-auto mb-2 opacity-50" />
        <p>暂无照片，请从左侧选择来源导入</p>
      </div>
    </div>
    
    <div v-else class="grid grid-cols-4 md:grid-cols-6 lg:grid-cols-8 gap-2">
      <div
        v-for="photo in photos"
        :key="photo.id"
        class="relative aspect-square rounded-lg overflow-hidden cursor-pointer group"
        :class="{
          'ring-2 ring-blue-500': selectedPhotos.includes(photo.id),
          'opacity-50': photo.isDuplicate,
        }"
        @click="toggleSelect(photo.id)"
        @dblclick="previewPhoto(photo)"
      >
        <img
          :src="`file://${photo.path}`"
          :alt="photo.fileName"
          class="w-full h-full object-cover"
        />
        <div class="absolute inset-0 bg-black bg-opacity-0 group-hover:bg-opacity-30 transition-colors flex items-center justify-center">
          <Check v-if="selectedPhotos.includes(photo.id)" class="w-8 h-8 text-white" />
          <Eye v-else class="w-6 h-6 text-white opacity-0 group-hover:opacity-100 transition-opacity" />
        </div>
        <div v-if="photo.isDuplicate" class="absolute top-1 right-1 bg-red-500 text-white text-xs px-1 rounded">
          重复
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Check, Eye, Image as ImageIcon, Loader2 } from 'lucide-vue-next';
import { usePhotoStore } from '../stores/photoStore';
import type { PhotoItem } from '../types';

const store = usePhotoStore();
const { photos, selectedPhotos, isScanning, toggleSelect } = store;

function previewPhoto(photo: PhotoItem) {
  console.log('预览照片:', photo.fileName);
}
</script>