<script setup lang="ts">
import { NInputGroup, NInput, NButton, NInputGroupLabel, NIcon, NSpace } from 'naive-ui'
import { useMessage } from 'naive-ui'
const props = defineProps<{ url: string }>();
const full_url = `${location.origin}/image/${props.url}`
const markdown = `![](${full_url})`
const message = useMessage();
const copy = (text: string) => {
  navigator.clipboard.writeText(text).then(() => {
    message.success('复制成功');
  }).catch(() => {
    message.error('复制失败');
  });
}
</script>

<template>
  <n-space vertical>
    <div class="image" :style="`background-image: url(${full_url})`"></div>
    <n-input-group>
      <n-input-group-label>
        <n-icon>
          <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 1024 1024">
            <path
              d="M574 665.4a8.03 8.03 0 0 0-11.3 0L446.5 781.6c-53.8 53.8-144.6 59.5-204 0c-59.5-59.5-53.8-150.2 0-204l116.2-116.2c3.1-3.1 3.1-8.2 0-11.3l-39.8-39.8a8.03 8.03 0 0 0-11.3 0L191.4 526.5c-84.6 84.6-84.6 221.5 0 306s221.5 84.6 306 0l116.2-116.2c3.1-3.1 3.1-8.2 0-11.3L574 665.4zm258.6-474c-84.6-84.6-221.5-84.6-306 0L410.3 307.6a8.03 8.03 0 0 0 0 11.3l39.7 39.7c3.1 3.1 8.2 3.1 11.3 0l116.2-116.2c53.8-53.8 144.6-59.5 204 0c59.5 59.5 53.8 150.2 0 204L665.3 562.6a8.03 8.03 0 0 0 0 11.3l39.8 39.8c3.1 3.1 8.2 3.1 11.3 0l116.2-116.2c84.5-84.6 84.5-221.5 0-306.1zM610.1 372.3a8.03 8.03 0 0 0-11.3 0L372.3 598.7a8.03 8.03 0 0 0 0 11.3l39.6 39.6c3.1 3.1 8.2 3.1 11.3 0l226.4-226.4c3.1-3.1 3.1-8.2 0-11.3l-39.5-39.6z"
              fill="currentColor">
            </path>
          </svg>
        </n-icon>
      </n-input-group-label>
      <n-input type="text" :defaultValue="full_url" />
      <n-button type="primary" :onClick="() => copy(full_url)">复制</n-button>
    </n-input-group>
    <n-input-group>
      <n-input-group-label>
        <n-icon>
          <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 1024 1024">
            <path
              d="M854.6 288.6L639.4 73.4c-6-6-14.1-9.4-22.6-9.4H192c-17.7 0-32 14.3-32 32v832c0 17.7 14.3 32 32 32h640c17.7 0 32-14.3 32-32V311.3c0-8.5-3.4-16.7-9.4-22.7zM790.2 326H602V137.8L790.2 326zm1.8 562H232V136h302v216a42 42 0 0 0 42 42h216v494zM429 481.2c-1.9-4.4-6.2-7.2-11-7.2h-35c-6.6 0-12 5.4-12 12v272c0 6.6 5.4 12 12 12h27.1c6.6 0 12-5.4 12-12V582.1l66.8 150.2a12 12 0 0 0 11 7.1H524c4.7 0 9-2.8 11-7.1l66.8-150.6V758c0 6.6 5.4 12 12 12H641c6.6 0 12-5.4 12-12V486c0-6.6-5.4-12-12-12h-34.7c-4.8 0-9.1 2.8-11 7.2l-83.1 191l-83.2-191z"
              fill="currentColor"></path>
          </svg>
        </n-icon>
      </n-input-group-label>
      <n-input :defaultValue="markdown" />
      <n-button type="primary" :onClick="() => copy(markdown)">复制</n-button>
    </n-input-group>
  </n-space>
</template>

<style>
.image {
  width: 100%;
  height: 160px;
  background-size: cover;
  background-position: center;
}
</style>
