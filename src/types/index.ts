export interface PhotoItem {
  id: string;
  path: string;
  fileName: string;
  fileSize: number;
  hash: string;
  exif: {
    date: string;
    device: string;
    width: number;
    height: number;
  };
  category: {
    theme: string;
    date: string;
    deviceType: string;
    fileType: string;
  };
  isSelected: boolean;
  isDuplicate: boolean;
  duplicateGroupId: string | null;
}

export interface ImportRecord {
  id: string;
  fileName: string;
  hash: string;
  importTime: string;
  exportPath: string;
}

export interface DeviceInfo {
  id: string;
  name: string;
  type: 'sdcard' | 'android' | 'ios' | 'local';
  path: string;
}

export interface CategoryRule {
  theme: string;
  dateFormat: string;
  deviceTypeEnabled: boolean;
  fileTypeEnabled: boolean;
}