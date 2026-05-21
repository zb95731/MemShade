<template>
  <div class="w-72 bg-[rgba(15,23,42,0.95)] backdrop-blur-xl border-r border-[rgba(71,85,105,0.5)] flex flex-col h-full">
    <div class="p-4 border-b border-[rgba(71,85,105,0.3)]">
      <h3 class="font-semibold text-slate-200 mb-3 flex items-center gap-2">
        <div class="w-2 h-2 rounded-full bg-gradient-to-r from-indigo-400 to-purple-400 animate-pulse"></div>
        导入来源
      </h3>
      <div class="space-y-2">
        <button
          v-for="device in devices"
          :key="device.id"
          class="w-full px-3 py-2.5 text-left rounded-xl hover:bg-[rgba(99,102,241,0.1)] hover:border-[rgba(99,102,241,0.3)] border border-transparent transition-all duration-300 group"
          @click="selectDevice(device)"
        >
          <span class="flex items-center gap-3">
            <div class="p-2 rounded-lg bg-[rgba(99,102,241,0.15)] text-indigo-400 group-hover:bg-[rgba(99,102,241,0.25)] transition-colors">
              <component :is="getDeviceIcon(device.type)" class="w-4 h-4" />
            </div>
            <span class="text-slate-300 group-hover:text-slate-100 transition-colors">{{ device.name }}</span>
          </span>
        </button>
        <button
          class="w-full px-3 py-2.5 text-left rounded-xl hover:bg-[rgba(99,102,241,0.1)] hover:border-[rgba(99,102,241,0.3)] border border-transparent transition-all duration-300 group"
          @click="selectLocalFolder"
        >
          <span class="flex items-center gap-3">
            <div class="p-2 rounded-lg bg-[rgba(99,102,241,0.15)] text-indigo-400 group-hover:bg-[rgba(99,102,241,0.25)] transition-colors">
              <FolderOpen class="w-4 h-4" />
            </div>
            <span class="text-slate-300 group-hover:text-slate-100 transition-colors">本地文件夹</span>
          </span>
        </button>
      </div>
    </div>
    
    <div class="flex-1 p-4 overflow-auto">
      <h3 class="font-semibold text-slate-200 mb-3 flex items-center gap-2">
        <div class="w-2 h-2 rounded-full bg-gradient-to-r from-emerald-400 to-cyan-400"></div>
        待处理文件
      </h3>
      <div class="space-y-1.5 max-h-48 overflow-auto">
        <div
          v-for="photo in photos.slice(0, 10)"
          :key="photo.id"
          class="text-sm text-slate-400 truncate px-2 py-1.5 rounded-lg bg-[rgba(30,41,59,0.5)] hover:bg-[rgba(30,41,59,0.8)] transition-colors"
        >
          {{ photo.fileName }}
        </div>
        <div v-if="photos.length > 10" class="text-xs text-slate-500 px-2 py-1">
          ... 还有 {{ photos.length - 10 }} 个文件
        </div>
      </div>
    </div>

    <div class="p-4 border-t border-[rgba(71,85,105,0.3)] bg-[rgba(15,23,42,0.8)]">
      <h3 class="font-semibold text-slate-200 mb-3 flex items-center gap-2">
        <div class="w-2 h-2 rounded-full bg-gradient-to-r from-amber-400 to-orange-400"></div>
        分类规则
      </h3>
      <div class="space-y-3">
        <div>
          <label class="text-xs text-slate-400 block mb-1.5">主题名称</label>
          <input
            v-model="categoryRule.theme"
            type="text"
            class="w-full px-3 py-2 text-sm bg-[rgba(30,41,59,0.8)] border border-[rgba(71,85,105,0.5)] rounded-xl text-slate-200 placeholder-slate-500 focus:outline-none focus:border-indigo-400 focus:ring-2 focus:ring-indigo-400/20 transition-all"
            placeholder="输入主题名称"
          />
        </div>
        <div class="flex items-center gap-2 px-1">
          <input
            v-model="categoryRule.deviceTypeEnabled"
            type="checkbox"
            id="deviceType"
            class="w-4 h-4 rounded border-[rgba(71,85,105,0.5)] text-indigo-500 bg-[rgba(30,41,59,0.8)] focus:ring-indigo-500/20"
          />
          <label for="deviceType" class="text-xs text-slate-400 cursor-pointer select-none">按设备分类</label>
        </div>
        <div class="flex items-center gap-2 px-1">
          <input
            v-model="categoryRule.fileTypeEnabled"
            type="checkbox"
            id="fileType"
            class="w-4 h-4 rounded border-[rgba(71,85,105,0.5)] text-indigo-500 bg-[rgba(30,41,59,0.8)] focus:ring-indigo-500/20"
          />
          <label for="fileType" class="text-xs text-slate-400 cursor-pointer select-none">按文件类型分类</label>
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