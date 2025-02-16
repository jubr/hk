import { defineConfig } from 'vitepress'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "hk",
  description: "git hook manager",
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Environment Variables', link: '/environment_variables' },
    ],

    sidebar: [
      {
        text: 'Getting Started',
        link: '/getting_started',
      },
      {
        text: 'Environment Variables',
        link: '/environment_variables',
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/vuejs/vitepress' }
    ]
  }
})
