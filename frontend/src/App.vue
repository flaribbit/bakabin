<script setup lang="ts">
import { NConfigProvider, NSpace, NUpload, NUploadDragger, NIcon, NText, NSwitch, UploadInst, NMessageProvider, NGi, NGrid, NA } from 'naive-ui'
import { UploadFileInfo } from 'naive-ui'
import { ref, onMounted } from 'vue';
import ImageItem from './components/ImageItem.vue';
const convert_webp = ref(false);
const upload_ref = ref<UploadInst>();
const file_list = ref<UploadFileInfo[]>([]);
const uploaded_list = ref<string[]>([]);
const onFinish = ({
  file,
  event
}: {
  file: UploadFileInfo
  event?: ProgressEvent
}) => {
  if (!event || !event.target) { return file };
  uploaded_list.value.push((event.target as XMLHttpRequest).response);
  return file
};
onMounted(() => {
  window.addEventListener('paste', (event: ClipboardEvent) => {
    const files = event.clipboardData?.files;
    if (!files) return;
    for (let file of files) {
      file_list.value.push({
        id: Date.now().toString(),
        name: '剪贴板图片',
        status: 'pending',
        file: file,
      });
    }
    upload_ref.value?.submit();
  });
});
</script>

<template>
  <n-config-provider>
    <n-message-provider>
      <n-space vertical>
        <h1>上传图片</h1>
        <n-space>
          <n-switch v-model:value="convert_webp" />
          <n-text>转换为 webp 格式</n-text>
        </n-space>
        <n-upload multiple action="/upload" v-model:file-list="file_list" ref="upload_ref" @finish="onFinish">
          <n-upload-dragger>
            <div style="margin-bottom: 12px">
              <n-icon size="48" :depth="3">
                <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 24 24">
                  <path
                    d="M19.35 10.04A7.49 7.49 0 0 0 12 4C9.11 4 6.6 5.64 5.35 8.04A5.994 5.994 0 0 0 0 14c0 3.31 2.69 6 6 6h13c2.76 0 5-2.24 5-5c0-2.64-2.05-4.78-4.65-4.96zM19 18H6c-2.21 0-4-1.79-4-4c0-2.05 1.53-3.76 3.56-3.97l1.07-.11l.5-.95A5.469 5.469 0 0 1 12 6c2.62 0 4.88 1.86 5.39 4.43l.3 1.5l1.53.11A2.98 2.98 0 0 1 22 15c0 1.65-1.35 3-3 3zM8 13h2.55v3h2.9v-3H16l-4-4z"
                    fill="currentColor">
                  </path>
                </svg>
              </n-icon>
            </div>
            <div style="font-size: 16px">
              点击或者拖动图片文件到此处上传
            </div>
            <div style="font-size: 16px">
              或按 Ctrl+V 粘贴剪贴板图片
            </div>
          </n-upload-dragger>
        </n-upload>
      </n-space>
      <n-grid x-gap="12" y-gap="12" cols="2 640:3">
        <n-gi v-for="url in uploaded_list">
          <image-item :url="url" />
        </n-gi>
      </n-grid>
    </n-message-provider>
    <div style="text-align: center; margin-top: 20px;">
      <n-text>© 2023
        <n-a href="https://github.com/flaribbit/bakabin" target="_blank">
          <n-icon size="16">
            <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 496 512">
              <path
                d="M165.9 397.4c0 2-2.3 3.6-5.2 3.6c-3.3.3-5.6-1.3-5.6-3.6c0-2 2.3-3.6 5.2-3.6c3-.3 5.6 1.3 5.6 3.6zm-31.1-4.5c-.7 2 1.3 4.3 4.3 4.9c2.6 1 5.6 0 6.2-2s-1.3-4.3-4.3-5.2c-2.6-.7-5.5.3-6.2 2.3zm44.2-1.7c-2.9.7-4.9 2.6-4.6 4.9c.3 2 2.9 3.3 5.9 2.6c2.9-.7 4.9-2.6 4.6-4.6c-.3-1.9-3-3.2-5.9-2.9zM244.8 8C106.1 8 0 113.3 0 252c0 110.9 69.8 205.8 169.5 239.2c12.8 2.3 17.3-5.6 17.3-12.1c0-6.2-.3-40.4-.3-61.4c0 0-70 15-84.7-29.8c0 0-11.4-29.1-27.8-36.6c0 0-22.9-15.7 1.6-15.4c0 0 24.9 2 38.6 25.8c21.9 38.6 58.6 27.5 72.9 20.9c2.3-16 8.8-27.1 16-33.7c-55.9-6.2-112.3-14.3-112.3-110.5c0-27.5 7.6-41.3 23.6-58.9c-2.6-6.5-11.1-33.3 2.6-67.9c20.9-6.5 69 27 69 27c20-5.6 41.5-8.5 62.8-8.5s42.8 2.9 62.8 8.5c0 0 48.1-33.6 69-27c13.7 34.7 5.2 61.4 2.6 67.9c16 17.7 25.8 31.5 25.8 58.9c0 96.5-58.9 104.2-114.8 110.5c9.2 7.9 17 22.9 17 46.4c0 33.7-.3 75.4-.3 83.6c0 6.5 4.6 14.4 17.3 12.1C428.2 457.8 496 362.9 496 252C496 113.3 383.5 8 244.8 8zM97.2 352.9c-1.3 1-1 3.3.7 5.2c1.6 1.6 3.9 2.3 5.2 1c1.3-1 1-3.3-.7-5.2c-1.6-1.6-3.9-2.3-5.2-1zm-10.8-8.1c-.7 1.3.3 2.9 2.3 3.9c1.6 1 3.6.7 4.3-.7c.7-1.3-.3-2.9-2.3-3.9c-2-.6-3.6-.3-4.3.7zm32.4 35.6c-1.6 1.3-1 4.3 1.3 6.2c2.3 2.3 5.2 2.6 6.5 1c1.3-1.3.7-4.3-1.3-6.2c-2.2-2.3-5.2-2.6-6.5-1zm-11.4-14.7c-1.6 1-1.6 3.6 0 5.9c1.6 2.3 4.3 3.3 5.6 2.3c1.6-1.3 1.6-3.9 0-6.2c-1.4-2.3-4-3.3-5.6-2z"
                fill="currentColor"></path>
            </svg>
          </n-icon>
          flaribbit/bakabin
        </n-a>
      </n-text>
    </div>
  </n-config-provider>
</template>

<style></style>
