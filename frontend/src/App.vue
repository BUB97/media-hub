<script setup lang="ts">
import HelloWorld from './components/HelloWorld.vue'
</script>

<template>
  <div class="min-h-screen bg-gray-100 flex flex-col items-center justify-center">
    <div class="flex space-x-8 mb-8">
      <a href="https://vite.dev" target="_blank" class="hover:scale-110 transition-transform">
        <img src="/vite.svg" class="logo" alt="Vite logo" />
      </a>
      <a href="https://vuejs.org/" target="_blank" class="hover:scale-110 transition-transform">
        <img src="./assets/vue.svg" class="logo vue" alt="Vue logo" />
      </a>
    </div>
    <div class="bg-white rounded-lg shadow-lg p-8">
      <HelloWorld msg="Vite + Vue + Tailwind CSS" />
    </div>
    <div class="mt-8 text-center">
      <p class="text-green-600 font-semibold text-lg">🎉 Tailwind CSS 已成功集成！</p>
      <p class="text-gray-600 mt-2">现在你可以使用 Tailwind 的实用类来快速构建界面</p>
    </div>
  </div>
</template>

<style scoped>
.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: filter 300ms;
}
.logo:hover {
  filter: drop-shadow(0 0 2em #646cffaa);
}
.logo.vue:hover {
  filter: drop-shadow(0 0 2em #42b883aa);
}
</style>
