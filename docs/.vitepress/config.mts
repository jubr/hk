import { defineConfig } from 'vitepress'

import spec from "../cli/commands.json";

function getCommands(cmd): string[][] {
  const commands = [];
  for (const [name, sub] of Object.entries(cmd.subcommands)) {
    if (sub.hide) continue;
    commands.push(sub.full_cmd);
    commands.push(...getCommands(sub));
  }
  return commands;
}

const commands = getCommands(spec.cmd);

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
      { text: 'CLI Reference', link: '/cli', items: commands.map(cmd => ({ text: cmd.join(' '), link: `/cli/${cmd.join('/')}` })) },
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/vuejs/vitepress' }
    ],

    search: {
      provider: 'local',
    },
  }
})
