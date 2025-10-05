import type {ReactNode} from 'react';
import clsx from 'clsx';
import Heading from '@theme/Heading';
import styles from './styles.module.css';

type FeatureItem = {
  title: string;
  icon: string;
  description: ReactNode;
};

const FeatureList: FeatureItem[] = [
  {
    title: 'Build & Deploy',
    icon: '🔨',
    description: (
      <>
        Compile ink! smart contracts and deploy them to GLIN Network with a single command.
        Automatic gas estimation and deployment verification included.
      </>
    ),
  },
  {
    title: 'Contract Interaction',
    icon: '📞',
    description: (
      <>
        Query contract state and execute transactions directly from the CLI.
        Real-time event monitoring and transaction tracking built-in.
      </>
    ),
  },
  {
    title: 'TypeScript Generation',
    icon: '📝',
    description: (
      <>
        Auto-generate TypeScript types and React hooks from your contract ABI.
        Seamless frontend integration with type safety.
      </>
    ),
  },
  {
    title: 'Multi-Network Support',
    icon: '🌐',
    description: (
      <>
        Switch between testnet, mainnet, and local networks effortlessly.
        Pre-configured networks with custom RPC support.
      </>
    ),
  },
  {
    title: 'Contract Templates',
    icon: '📦',
    description: (
      <>
        Start from battle-tested templates: ERC20, ERC721, DAO, and more.
        Customize and deploy in minutes.
      </>
    ),
  },
  {
    title: 'Gas Optimization',
    icon: '⚡',
    description: (
      <>
        Built-in gas estimation with optimization tips.
        WASM size analysis and performance recommendations.
      </>
    ),
  },
];

function Feature({title, icon, description}: FeatureItem) {
  return (
    <div className={clsx('col col--4')}>
      <div className="text--center padding-vert--md">
        <div className={styles.featureIcon}>{icon}</div>
      </div>
      <div className="text--center padding-horiz--md">
        <Heading as="h3">{title}</Heading>
        <p>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures(): ReactNode {
  return (
    <section className={styles.features}>
      <div className="container">
        <div className="row">
          {FeatureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  );
}
