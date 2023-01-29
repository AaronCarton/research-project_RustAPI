<template>
  <div class="bg-[#5347F3]">
    <div class="bg-opacity-65 grid min-h-screen place-items-center bg-black">
      <div class="flex w-full max-w-lg flex-col px-6">
        <h1 class="font-title mb-24 self-center text-center text-8xl text-neutral-50">Quiz</h1>

        <div class="mb-20 rounded-md">
          <router-view></router-view>
        </div>
      </div>
    </div>
  </div>
</template>
