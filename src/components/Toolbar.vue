<template>
  <div class="h-12 bg-white border-b border-gray-200 flex items-center justify-between px-4">
    <div class="flex items-center gap-2">
      <button
        class="flex items-center gap-1 px-3 py-1.5 rounded hover:bg-gray-100 transition-colors"
        @click="selectAll"
      >
        <CheckSquare class="w-4 h-4" />
        <span class="text-sm">全选</span>
      </button>
      <button
        class="flex items-center gap-1 px-3 py-1.5 rounded hover:bg-gray-100 transition-colors"
        @click="clearSelection"
      >
        <Square class="w-4 h-4" />
        <span class="text-sm">取消选择</span>
      </button>
      <div class="w-px h-6 bg-gray-200 mx-2"></div>
      <button
        class="flex items-center gap-1 px-3 py-1.5 rounded hover:bg-gray-100 transition-colors"
        @click="showDuplicates"
      >
        <AlertCircle class="w-4 h-4 text-red-500" />
        <span class="text-sm">显示重复 ({{ duplicateCount }})</span>
      </button>
    </div>

    <div class="flex items-center gap-2">
      <button
        class="flex items-center gap-1 px-3 py-1.5 rounded hover:bg-gray-100 transition-colors"
        @click="previewSelected"
      >
        <Eye class="w-4 h-4" />
        <span class="text-sm">预览</span>
      </button>
      <div class="w-px h-6 bg-gray-200 mx-2"></div>
      <button
        class="flex items-center gap-1 px-4 py-1.5 rounded bg-blue-500 text-white hover:bg-blue-600 transition-colors"
        @click="startExport"
      >
        <Download class="w-4 h-4" />
        <span class="text-sm">导出</span>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { CheckSquare, Square, AlertCircle, Eye, Download } from 'lucide-vue-next';
import { usePhotoStore } from '../stores/photoStore';
import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';

const store = usePhotoStore();
const { selectAll, clearSelection, selectedPhotos, photos, duplicateCount, categoryRule, setExportPath } = store;

function showDuplicates() {
  console.log('显示重复照片');
}

function previewSelected() {
  console.log('预览选中照片');
}

async function startExport() {
  if (selectedPhotos.length === 0) {
    alert('请先选择要导出的照片');
    return;
  }

  if (!categoryRule.theme) {
    alert('请先设置主题名称');
    return;
  }

  const path = await open({
    directory: true,
    multiple: false,
  });

  if (!path) return;

  const exportPathStr = Array.isArray(path) ? path[0] : path;
  setExportPath(exportPathStr);

  const selectedItems = photos.filter(p => selectedPhotos.includes(p.id));

  try {
    for (const item of selectedItems) {
      const date = item.exif.date || '未知日期';
      const year = date.substring(0, 4) + '年';
      const dateDir = date.substring(0, 8) || '未知日期';
      const themeDir = `${dateDir}_${categoryRule.theme}`;
      const deviceType = item.category.deviceType || '其他';
      const fileType = item.fileName.toLowerCase().endsWith('mp4') || item.fileName.toLowerCase().endsWith('mov') ? '视频' : '照片';

      let destDir = `${exportPathStr}\\${year}\\${themeDir}`;
      if (categoryRule.deviceTypeEnabled && categoryRule.fileTypeEnabled) {
        destDir += `\\${deviceType}${fileType}`;
      } else if (categoryRule.deviceTypeEnabled) {
        destDir += `\\${deviceType}`;
      } else if (categoryRule.fileTypeEnabled) {
        destDir += `\\${fileType}`;
      }

      await invoke('create_directory', { path: destDir });
      await invoke('copy_files', { files: [[item.path, `${destDir}\\${item.fileName}`]] });
      await invoke('insert_import_record', {
        fileName: item.fileName,
        hash: item.hash,
        exportPath: destDir,
      });
    }

    alert('导出完成！');
  } catch (error) {
    console.error('导出失败:', error);
    alert('导出失败，请重试');
  }
}
</script>