import { defineConfig } from 'vitepress'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "hk",
  description: "git hook manager",
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: 'Getting Started', link: '/getting_started' },
      { text: 'Configuration', link: '/configuration' },
      { text: 'GitHub', link: 'https://github.com/jdx/hk' },
      { text: 'Discord', link: 'https://discord.gg/UBa7pJUN7Z' },
    ],

    sidebar: [
      { text: 'Getting Started', link: '/getting_started' },
      { text: 'Configuration', link: '/configuration' },
      { text: 'Environment Variables', link: '/environment_variables' },
      { text: 'About', link: '/about' },
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/vuejs/vitepress' }
    ]
  }
})
