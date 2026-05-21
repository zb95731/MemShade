<template>
  <div class="flex-1 overflow-auto p-6">
    <div v-if="isScanning" class="flex items-center justify-center h-full min-h-96">
      <div class="text-center animate-fade-in">
        <div class="relative">
          <Loader2 class="w-16 h-16 animate-spin text-indigo-400 mx-auto" />
          <div class="absolute inset-0 flex items-center justify-center">
            <div class="w-8 h-8 rounded-full bg-gradient-to-br from-indigo-500 to-purple-500 animate-pulse"></div>
          </div>
        </div>
        <p class="mt-6 text-slate-300 text-lg">正在扫描文件...</p>
        <p class="mt-2 text-slate-500 text-sm">请稍候</p>
      </div>
    </div>
    
    <div v-else-if="photos.length === 0" class="flex items-center justify-center h-full min-h-96">
      <div class="text-center animate-fade-in">
        <div class="w-24 h-24 mx-auto mb-6 rounded-3xl bg-gradient-to-br from-[rgba(99,102,241,0.15)] to-[rgba(168,85,247,0.15)] flex items-center justify-center">
          <ImageIcon class="w-12 h-12 text-indigo-400" />
        </div>
        <h3 class="text-xl font-semibold text-slate-200 mb-2">暂无照片</h3>
        <p class="text-slate-500">请从左侧选择来源导入照片</p>
      </div>
    </div>
    
    <div v-else class="grid grid-cols-3 sm:grid-cols-4 md:grid-cols-5 lg:grid-cols-6 xl:grid-cols-8 gap-4 animate-fade-in">
      <div
        v-for="(photo, index) in photos"
        :key="photo.id"
        class="relative aspect-square rounded-2xl overflow-hidden cursor-pointer group animate-fade-in"
        style="animation-delay: calc(0.02s * var(--index));"
        :class="{
          'ring-2 ring-indigo-400 ring-offset-2 ring-offset-[#0f172a]': selectedPhotos.includes(photo.id),
          'opacity-60 grayscale-[0.3]': photo.isDuplicate,
        }"
        :style="{ '--index': index }"
        @click="toggleSelect(photo.id)"
        @dblclick="previewPhoto(photo)"
      >
        <img
          :src="`file://${photo.path}`"
          :alt="photo.fileName"
          class="w-full h-full object-cover transition-transform duration-500 group-hover:scale-110"
          loading="lazy"
        />
        <div class="absolute inset-0 bg-gradient-to-t from-black/60 via-black/0 to-black/0 opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
        <div class="absolute inset-0 bg-black/0 group-hover:bg-black/20 transition-colors duration-300 flex items-center justify-center">
          <div v-if="selectedPhotos.includes(photo.id)" class="w-14 h-14 rounded-full bg-gradient-to-br from-indigo-500 to-purple-500 flex items-center justify-center shadow-lg shadow-indigo-500/30 animate-scale-in">
            <Check class="w-7 h-7 text-white" />
          </div>
          <div v-else class="w-12 h-12 rounded-full bg-white/20 backdrop-blur-md flex items-center justify-center opacity-0 group-hover:opacity-100 transition-all duration-300 transform scale-80 group-hover:scale-100">
            <Eye class="w-6 h-6 text-white" />
          </div>
        </div>
        <div v-if="photo.isDuplicate" class="absolute top-3 right-3 px-2.5 py-1 rounded-full bg-gradient-to-r from-red-500 to-orange-500 text-white text-xs font-medium shadow-lg shadow-red-500/30">
          重复
        </div>
        <div class="absolute bottom-0 left-0 right-0 p-3 opacity-0 group-hover:opacity-100 transition-opacity duration-300">
          <p class="text-white text-xs font-medium truncate drop-shadow-lg">{{ photo.fileName }}</p>
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