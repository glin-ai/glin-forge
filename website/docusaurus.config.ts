import {themes as prismThemes} from 'prism-react-renderer';
import type {Config} from '@docusaurus/types';
import type * as Preset from '@docusaurus/preset-classic';

const config: Config = {
  title: 'GLIN Forge',
  tagline: 'Smart contract development CLI for GLIN Network',
  favicon: 'img/favicon.ico',

  future: {
    v4: true,
  },

  url: 'https://glinforge.com',
  baseUrl: '/',

  organizationName: 'glin-ai',
  projectName: 'glin-forge',

  onBrokenLinks: 'throw',
  onBrokenMarkdownLinks: 'warn',

  i18n: {
    defaultLocale: 'en',
    locales: ['en'],
  },

  presets: [
    [
      'classic',
      {
        docs: {
          sidebarPath: './sidebars.ts',
          editUrl: 'https://github.com/glin-ai/glin-forge/tree/main/website/',
          routeBasePath: '/', // Docs-only mode
        },
        blog: false, // Disable blog
        theme: {
          customCss: './src/css/custom.css',
        },
      } satisfies Preset.Options,
    ],
  ],

  themeConfig: {
    image: 'img/glin-forge-social.png',
    colorMode: {
      defaultMode: 'light',
      disableSwitch: false,
      respectPrefersColorScheme: true,
    },
    navbar: {
      title: 'GLIN Forge',
      logo: {
        alt: 'GLIN Forge Logo',
        src: 'img/logo.svg',
        srcDark: 'img/logo-dark.svg',
      },
      items: [
        {
          to: '/getting-started/installation',
          label: 'Getting Started',
          position: 'left',
        },
        {
          to: '/cli-reference/overview',
          label: 'CLI Reference',
          position: 'left',
        },
        {
          to: '/guides/building-contracts',
          label: 'Guides',
          position: 'left',
        },
        {
          to: '/examples/erc20-token',
          label: 'Examples',
          position: 'left',
        },
        {
          href: 'https://github.com/glin-ai/glin-forge',
          label: 'GitHub',
          position: 'right',
        },
      ],
    },
    footer: {
      style: 'dark',
      links: [
        {
          title: 'Docs',
          items: [
            {
              label: 'Getting Started',
              to: '/getting-started/installation',
            },
            {
              label: 'CLI Reference',
              to: '/cli-reference/overview',
            },
            {
              label: 'Guides',
              to: '/guides/building-contracts',
            },
            {
              label: 'Examples',
              to: '/examples/erc20-token',
            },
          ],
        },
        {
          title: 'Community',
          items: [
            {
              label: 'Discord',
              href: 'https://discord.gg/glin-ai',
            },
            {
              label: 'GitHub',
              href: 'https://github.com/glin-ai/glin-forge',
            },
            {
              label: 'Twitter',
              href: 'https://twitter.com/glin_ai',
            },
          ],
        },
        {
          title: 'GLIN Ecosystem',
          items: [
            {
              label: 'GLIN Network',
              href: 'https://glin.ai',
            },
            {
              label: 'Network Docs',
              href: 'https://docs.glin.ai',
            },
            {
              label: 'Explorer',
              href: 'https://glinscan.com',
            },
            {
              label: 'SDK',
              href: 'https://docs.glin.ai/sdk/intro',
            },
          ],
        },
        {
          title: 'More',
          items: [
            {
              label: 'Troubleshooting',
              to: '/troubleshooting/common-errors',
            },
            {
              label: 'Report Issue',
              href: 'https://github.com/glin-ai/glin-forge/issues',
            },
          ],
        },
      ],
      copyright: `Copyright © ${new Date().getFullYear()} GLIN AI. Apache-2.0 License.`,
    },
    prism: {
      theme: prismThemes.github,
      darkTheme: prismThemes.dracula,
      additionalLanguages: ['rust', 'toml', 'typescript', 'bash', 'json'],
    },
    metadata: [
      {name: 'keywords', content: 'glin, smart contracts, ink, substrate, polkadot, blockchain, cli, rust'},
      {name: 'description', content: 'Official CLI tool for developing, deploying, and interacting with ink! smart contracts on GLIN Network'},
    ],
  } satisfies Preset.ThemeConfig,
};

export default config;
