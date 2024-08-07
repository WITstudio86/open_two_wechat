/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.rs"],
  theme: {
    extend: {},
  },
  plugins: [
    require('daisyui')
  ],
  daisyui: {
    themes: false, // 为 True 包含所有主题 , 为 false 只有白天和黑暗 , 数组就加载数组中的所有 , 第一个为默认
    darkTheme: "dark", // 设置暗黑模式下使用的主题 
    base: true, // 是否启用一些基本原始
    styled: true, // 设置为 true 会有默认颜色或样式否则没有
    utils: true, // 响应类工具会被添加
    prefix: "", // 给 daisyUI 类名添加前缀名
    logs: true, // 为 true 会在 css 构建的时候在命令行窗口输出日志
    themeRoot: ":root", // 将主题 CSS 变量附加到哪个元素。在某些情况下（例如在 shadow root 中嵌入 daisyUI），将其设置为 * 可能很有用，因此所有组件都可以访问所需的 CSS 变量。
  },
}

