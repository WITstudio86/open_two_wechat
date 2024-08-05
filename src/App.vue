<template>
  <div class="h-screen flex flex-col justify-around items-center">
    <h1 class="text-3xl font-bold text-center">微信双开</h1>

    <p class="w-3/5">
      点击后如果无效 , 请将运行结果及异常运行情况
      <a class="link-primary" href="mailto:email@example.com">发送到邮件</a>
    </p>

    <div class="tooltip" data-tip="为确保运行顺利 点击运行前先确保:当前所有已打开微信已经退出账号并关闭">
      <button class="btn btn-primary" @click="run">
        运行
      </button>
      <dialog id="my_modal_1" class="modal">
        <div class="modal-box">
          <h3 class="text-lg font-bold">运行结束</h3>
          <p class="py-4">{{ message }}</p>
          <div class="modal-action">
            <form method="dialog">
              <!-- if there is a button in form, it will close the modal -->
              <button class="btn">Close</button>
            </form>
          </div>
        </div>
      </dialog>
    </div>
    <footer class="text-left w-screen text-sm text-gray-500">
      <p>© 2024 WITStudio 网络开发工作室 微信双开</p>
      <p> &rarrhk; 原型demo No.802 </p>
    </footer>
  </div>

</template>

<script setup>
import { invoke } from '@tauri-apps/api';
import { ref } from 'vue';

let message = ref('')

async function run() {
  my_modal_1.showModal()
  // 通过 navigator.userAgent 获取用户代理字符串
  const userAgent = window.navigator.userAgent;
  // 使用正则表达式匹配操作系统信息
  const platform = /(Mac|Win|Linux)/.test(userAgent) ? userAgent.charAt(0).toUpperCase() + userAgent.slice(1) : 'Unknown';

  message.value = await invoke("show", { os: platform })

  my_modal_1.showModal()
}
</script>