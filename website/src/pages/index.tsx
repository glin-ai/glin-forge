import type {ReactNode} from 'react';
import clsx from 'clsx';
import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import HomepageFeatures from '@site/src/components/HomepageFeatures';
import Heading from '@theme/Heading';

import styles from './index.module.css';

function HomepageHeader() {
  const {siteConfig} = useDocusaurusContext();
  return (
    <header className={clsx('hero hero--primary', styles.heroBanner)}>
      <div className="container">
        <Heading as="h1" className="hero__title">
          {siteConfig.title}
        </Heading>
        <p className="hero__subtitle">{siteConfig.tagline}</p>
        <div className={styles.buttons}>
          <Link
            className="button button--secondary button--lg"
            to="/getting-started/installation">
            Get Started →
          </Link>
          <Link
            className="button button--outline button--lg"
            to="/cli-reference/overview"
            style={{marginLeft: '1rem'}}>
            CLI Reference
          </Link>
        </div>
        <div className={styles.codePreview}>
          <pre className={styles.code}>
            {`# Create a new smart contract
glin-forge new my-token --template erc20

# Build and deploy
glin-forge build --release
glin-forge deploy --network testnet --account alice

# Generate TypeScript types
glin-forge typegen --output ./frontend/src/types`}
          </pre>
        </div>
      </div>
    </header>
  );
}

export default function Home(): ReactNode {
  const {siteConfig} = useDocusaurusContext();
  return (
    <Layout
      title="Smart Contract Development CLI"
      description="Official CLI tool for developing, deploying, and interacting with ink! smart contracts on GLIN Network">
      <HomepageHeader />
      <main>
        <HomepageFeatures />
      </main>
    </Layout>
  );
}
