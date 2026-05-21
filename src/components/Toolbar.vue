<template>
  <div class="h-16 bg-[rgba(30,41,59,0.9)] backdrop-blur-xl border-b border-[rgba(71,85,105,0.5)] flex items-center justify-between px-6 shadow-soft">
    <div class="flex items-center gap-3">
      <div class="flex items-center gap-3 mr-8">
        <div class="w-10 h-10 rounded-xl bg-gradient-to-br from-[#6366f1] to-[#8b5cf6] flex items-center justify-center shadow-glow">
          <svg class="w-5 h-5 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
          </svg>
        </div>
        <span class="font-semibold text-lg text-[#f8fafc]">PhotoOrganizer</span>
      </div>

      <div class="h-8 w-px bg-[rgba(71,85,105,0.5)] mx-2"></div>

      <button
        class="flex items-center gap-2 px-4 py-2 rounded-xl bg-[rgba(51,65,85,0.5)] hover:bg-[rgba(51,65,85,0.8)] transition-all duration-300 border border-transparent hover:border-[rgba(99,102,241,0.3)] group"
        @click="selectAll"
      >
        <CheckSquare class="w-4 h-4 text-[#cbd5e1] group-hover:text-[#6366f1] transition-colors" />
        <span class="text-sm font-medium text-[#cbd5e1] group-hover:text-[#f8fafc]">全选</span>
      </button>

      <button
        class="flex items-center gap-2 px-4 py-2 rounded-xl bg-[rgba(51,65,85,0.5)] hover:bg-[rgba(51,65,85,0.8)] transition-all duration-300 border border-transparent hover:border-[rgba(148,163,184,0.3)] group"
        @click="clearSelection"
      >
        <Square class="w-4 h-4 text-[#cbd5e1] group-hover:text-[#94a3b8] transition-colors" />
        <span class="text-sm font-medium text-[#cbd5e1] group-hover:text-[#f8fafc]">取消选择</span>
      </button>

      <div class="h-8 w-px bg-[rgba(71,85,105,0.5)] mx-2"></div>

      <button
        class="flex items-center gap-2 px-4 py-2 rounded-xl bg-[rgba(239,68,68,0.1)] hover:bg-[rgba(239,68,68,0.2)] transition-all duration-300 border border-transparent hover:border-[rgba(239,68,68,0.3)] group"
        @click="showDuplicates"
      >
        <AlertCircle class="w-4 h-4 text-[#ef4444]" />
        <span class="text-sm font-medium text-[#ef4444]">显示重复 ({{ duplicateCount }})</span>
      </button>
    </div>

    <div class="flex items-center gap-3">
      <button
        class="flex items-center gap-2 px-4 py-2 rounded-xl bg-[rgba(51,65,85,0.5)] hover:bg-[rgba(51,65,85,0.8)] transition-all duration-300 border border-transparent hover:border-[rgba(148,163,184,0.3)] group"
        @click="previewSelected"
      >
        <Eye class="w-4 h-4 text-[#cbd5e1] group-hover:text-[#14b8a6] transition-colors" />
        <span class="text-sm font-medium text-[#cbd5e1] group-hover:text-[#f8fafc]">预览</span>
      </button>

      <div class="h-8 w-px bg-[rgba(71,85,105,0.5)] mx-2"></div>

      <button
        class="flex items-center gap-2 px-6 py-2.5 rounded-xl btn-primary text-white font-medium"
        @click="startExport"
      >
        <Download class="w-4 h-4" />
        <span>导出</span>
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
