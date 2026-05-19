<template>
  <div class="w-64 bg-gray-100 border-r border-gray-200 flex flex-col h-full">
    <div class="p-3 border-b border-gray-200">
      <h3 class="font-semibold text-gray-700 mb-2">来源</h3>
      <div class="space-y-1">
        <button
          v-for="device in devices"
          :key="device.id"
          class="w-full px-3 py-2 text-left rounded hover:bg-gray-200 transition-colors"
          @click="selectDevice(device)"
        >
          <span class="flex items-center gap-2">
            <component :is="getDeviceIcon(device.type)" class="w-4 h-4" />
            {{ device.name }}
          </span>
        </button>
        <button
          class="w-full px-3 py-2 text-left rounded hover:bg-gray-200 transition-colors"
          @click="selectLocalFolder"
        >
          <FolderOpen class="w-4 h-4 inline mr-2" />
          本地文件夹
        </button>
      </div>
    </div>
    
    <div class="flex-1 p-3 overflow-auto">
      <h3 class="font-semibold text-gray-700 mb-2">待处理文件</h3>
      <div class="space-y-1 max-h-48 overflow-auto">
        <div
          v-for="photo in photos.slice(0, 10)"
          :key="photo.id"
          class="text-sm text-gray-600 truncate"
        >
          {{ photo.fileName }}
        </div>
        <div v-if="photos.length > 10" class="text-xs text-gray-400">
          ... 还有 {{ photos.length - 10 }} 个文件
        </div>
      </div>
    </div>

    <div class="p-3 border-t border-gray-200">
      <h3 class="font-semibold text-gray-700 mb-2">分类规则</h3>
      <div class="space-y-2">
        <div>
          <label class="text-xs text-gray-500 block mb-1">主题名称</label>
          <input
            v-model="categoryRule.theme"
            type="text"
            class="w-full px-2 py-1 text-sm border rounded"
            placeholder="输入主题名称"
          />
        </div>
        <div class="flex items-center gap-2">
          <input
            v-model="categoryRule.deviceTypeEnabled"
            type="checkbox"
            id="deviceType"
          />
          <label for="deviceType" class="text-xs text-gray-600">按设备分类</label>
        </div>
        <div class="flex items-center gap-2">
          <input
            v-model="categoryRule.fileTypeEnabled"
            type="checkbox"
            id="fileType"
          />
          <label for="fileType" class="text-xs text-gray-600">按文件类型分类</label>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { FolderOpen, Smartphone, Camera, HardDrive } from 'lucide-vue-next';
import { usePhotoStore } from '../stores/photoStore';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';

const store = usePhotoStore();
const { photos, devices, categoryRule } = store;

function getDeviceIcon(type: string) {
  switch (type) {
    case 'sdcard': return Camera;
    case 'android':
    case 'ios': return Smartphone;
    default: return HardDrive;
  }
}

async function selectDevice(device: typeof devices[0]) {
  await scanPath(device.path);
}

async function selectLocalFolder() {
  const path = await open({
    directory: true,
    multiple: false,
  });
  if (path) {
    await scanPath(Array.isArray(path) ? path[0] : path);
  }
}

async function scanPath(path: string) {
  store.setIsScanning(true);
  try {
    const files = await invoke('scan_files', { path }) as string[];
    const newPhotos = await Promise.all(
      files.map(async (filePath) => {
        const hash = await invoke('compute_hash', { filePath }) as string;
        const isDuplicate = await invoke('check_duplicate', { hash }) as boolean;
        const fileName = filePath.split('\\').pop() || filePath.split('/').pop() || '';
        
        return {
          id: crypto.randomUUID(),
          path: filePath,
          fileName,
          fileSize: 0,
          hash,
          exif: {
            date: '',
            device: '',
            width: 0,
            height: 0,
          },
          category: {
            theme: '',
            date: '',
            deviceType: '',
            fileType: '',
          },
          isSelected: false,
          isDuplicate,
          duplicateGroupId: null,
        };
      })
    );
    store.addPhotos(newPhotos);
  } catch (error) {
    console.error('扫描失败:', error);
  } finally {
    store.setIsScanning(false);
  }
}
</script>