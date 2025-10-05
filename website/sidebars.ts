import type {SidebarsConfig} from '@docusaurus/plugin-content-docs';

const sidebars: SidebarsConfig = {
  docs: [
    'intro',
    {
      type: 'category',
      label: 'Getting Started',
      link: {type: 'generated-index'},
      items: [
        'getting-started/installation',
        'getting-started/prerequisites',
        'getting-started/quick-start',
        'getting-started/first-contract',
      ],
    },
    {
      type: 'category',
      label: 'CLI Reference',
      link: {type: 'doc', id: 'cli-reference/overview'},
      items: [
        {
          type: 'category',
          label: 'Project Commands',
          items: [
            'cli-reference/project/new',
            'cli-reference/project/init',
            'cli-reference/project/build',
            'cli-reference/project/test',
          ],
        },
        {
          type: 'category',
          label: 'Deployment Commands',
          items: [
            'cli-reference/deployment/deploy',
            'cli-reference/deployment/upload',
            'cli-reference/deployment/instantiate',
          ],
        },
        {
          type: 'category',
          label: 'Interaction Commands',
          items: [
            'cli-reference/interaction/query',
            'cli-reference/interaction/call',
            'cli-reference/interaction/watch',
          ],
        },
        {
          type: 'category',
          label: 'Code Generation',
          items: [
            'cli-reference/codegen/typegen',
          ],
        },
        {
          type: 'category',
          label: 'Verification',
          items: [
            'cli-reference/verification/verify',
          ],
        },
        {
          type: 'category',
          label: 'Configuration',
          items: [
            'cli-reference/config/manage-config',
            'cli-reference/config/network',
            'cli-reference/config/account',
            'cli-reference/config/balance',
          ],
        },
      ],
    },
    {
      type: 'category',
      label: 'Guides',
      link: {type: 'generated-index'},
      items: [
        'guides/building-contracts',
        'guides/deploying-testnet',
        'guides/deploying-mainnet',
        'guides/frontend-integration',
        'guides/contract-interaction',
        'guides/event-handling',
        'guides/gas-optimization',
        'guides/multi-network',
        'guides/account-management',
        'guides/testing-strategies',
      ],
    },
    {
      type: 'category',
      label: 'Templates',
      link: {type: 'doc', id: 'templates/overview'},
      items: [
        'templates/erc20',
        'templates/erc721',
        'templates/dao',
        'templates/custom-templates',
      ],
    },
    {
      type: 'category',
      label: 'Code Generation',
      link: {type: 'doc', id: 'code-generation/overview'},
      items: [
        'code-generation/typescript-types',
        'code-generation/react-hooks',
        'code-generation/type-mapping',
      ],
    },
    {
      type: 'category',
      label: 'Configuration',
      link: {type: 'generated-index'},
      items: [
        'configuration/config-file',
        'configuration/networks',
        'configuration/accounts',
        'configuration/environment-variables',
      ],
    },
    {
      type: 'category',
      label: 'Advanced',
      link: {type: 'generated-index'},
      items: [
        'advanced/gas-estimation',
        'advanced/contract-verification',
        'advanced/deterministic-deploy',
        'advanced/factory-pattern',
        'advanced/metadata-parsing',
        'advanced/scale-encoding',
        'advanced/custom-networks',
      ],
    },
    {
      type: 'category',
      label: 'Examples',
      link: {type: 'generated-index'},
      items: [
        'examples/erc20-token',
        'examples/nft-collection',
        'examples/dao-governance',
        'examples/dex-contract',
        'examples/frontend-dapp',
      ],
    },
    {
      type: 'category',
      label: 'Troubleshooting',
      link: {type: 'generated-index'},
      items: [
        'troubleshooting/common-errors',
        'troubleshooting/network-issues',
        'troubleshooting/build-errors',
        'troubleshooting/deployment-errors',
        'troubleshooting/gas-errors',
      ],
    },
    {
      type: 'category',
      label: 'Reference',
      link: {type: 'generated-index'},
      items: [
        'reference/networks',
        'reference/error-codes',
        'reference/changelog',
      ],
    },
  ],
};

export default sidebars;
