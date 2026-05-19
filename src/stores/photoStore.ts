import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { PhotoItem, DeviceInfo, CategoryRule } from '../types';

export const usePhotoStore = defineStore('photos', () => {
  const photos = ref<PhotoItem[]>([]);
  const selectedPhotos = ref<string[]>([]);
  const devices = ref<DeviceInfo[]>([]);
  const categoryRule = ref<CategoryRule>({
    theme: '',
    dateFormat: 'YYYY年/YYYYMMDD_主题',
    deviceTypeEnabled: true,
    fileTypeEnabled: true,
  });
  const exportPath = ref('');
  const isScanning = ref(false);

  const totalCount = computed(() => photos.value.length);
  const selectedCount = computed(() => selectedPhotos.value.length);
  const duplicateCount = computed(() => photos.value.filter(p => p.isDuplicate).length);

  function addPhotos(newPhotos: PhotoItem[]) {
    photos.value = [...photos.value, ...newPhotos];
  }

  function toggleSelect(id: string) {
    const index = selectedPhotos.value.indexOf(id);
    if (index > -1) {
      selectedPhotos.value.splice(index, 1);
    } else {
      selectedPhotos.value.push(id);
    }
  }

  function selectAll() {
    selectedPhotos.value = photos.value.map(p => p.id);
  }

  function clearSelection() {
    selectedPhotos.value = [];
  }

  function updateCategoryRule(rule: Partial<CategoryRule>) {
    categoryRule.value = { ...categoryRule.value, ...rule };
  }

  function setExportPath(path: string) {
    exportPath.value = path;
  }

  function setIsScanning(scanning: boolean) {
    isScanning.value = scanning;
  }

  function clearPhotos() {
    photos.value = [];
    selectedPhotos.value = [];
  }

  return {
    photos,
    selectedPhotos,
    devices,
    categoryRule,
    exportPath,
    isScanning,
    totalCount,
    selectedCount,
    duplicateCount,
    addPhotos,
    toggleSelect,
    selectAll,
    clearSelection,
    updateCategoryRule,
    setExportPath,
    setIsScanning,
    clearPhotos,
  };
});