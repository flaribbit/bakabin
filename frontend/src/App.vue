<script setup lang="ts">
import { NConfigProvider, NSpace, NUpload, NUploadDragger, NIcon, NText, NSwitch, UploadInst, NMessageProvider } from 'naive-ui'
import { UploadFileInfo } from 'naive-ui'
import { ref, onMounted } from 'vue';
import ImageItem from './components/ImageItem.vue';
const convert_webp = ref(false);
const upload_ref = ref<UploadInst>();
const file_list = ref<UploadFileInfo[]>([]);
const test_url = "422a9d4d-0e57-48d4-b11c-26ba4cb20e9f.jpg";
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
        <n-space>
          <n-switch v-model:value="convert_webp" />
          <n-text>转换为 webp 格式</n-text>
        </n-space>
        <n-upload multiple action="/upload" v-model:file-list="file_list" ref="upload_ref" style="min-width: 480px;"
          @finish="onFinish">
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
      <n-space vertical>
        <image-item v-for="url in uploaded_list" :url="url" />
      </n-space>
    </n-message-provider>
  </n-config-provider>
</template>

<style></style>
