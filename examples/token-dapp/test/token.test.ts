import { expect } from 'chai';
import {
  deploy,
  getTestAccounts,
  expectEvent,
  expectSuccess,
  createBalanceTracker,
  initTesting,
} from '@glin-forge/testing';
import { initApi } from '@glin-forge/sdk';

describe('Token Contract', () => {
  let api: any;
  let contract: any;
  let alice: any;
  let bob: any;
  let charlie: any;

  const INITIAL_SUPPLY = 1000000;

  before(async () => {
    api = await initApi();
    await initTesting(api);

    const accounts = await getTestAccounts();
    alice = accounts.alice;
    bob = accounts.bob;
    charlie = accounts.charlie;

    // Deploy contract
    const result = await deploy('token', {
      constructorArgs: [INITIAL_SUPPLY],
      signer: alice,
    });

    contract = result.contract;
  });

  after(async () => {
    await api.disconnect();
  });

  describe('Deployment', () => {
    it('should set the correct total supply', async () => {
      const totalSupply = await contract.query.totalSupply();
      expect(totalSupply.output.toNumber()).to.equal(INITIAL_SUPPLY);
    });

    it('should assign all tokens to the deployer', async () => {
      const balance = await contract.query.balanceOf(alice.address);
      expect(balance.output.toNumber()).to.equal(INITIAL_SUPPLY);
    });
  });

  describe('Transfer', () => {
    it('should transfer tokens between accounts', async () => {
      const tracker = createBalanceTracker();
      await tracker.before(bob);

      const result = await contract.tx.transfer(bob.address, 1000, {
        signer: alice,
      });

      expectSuccess(result);
      expectEvent(result, 'contracts', 'ContractEmitted');

      const aliceBalance = await contract.query.balanceOf(alice.address);
      const bobBalance = await contract.query.balanceOf(bob.address);

      expect(aliceBalance.output.toNumber()).to.equal(INITIAL_SUPPLY - 1000);
      expect(bobBalance.output.toNumber()).to.equal(1000);
    });

    it('should fail when transferring more than balance', async () => {
      const result = await contract.tx.transfer(charlie.address, INITIAL_SUPPLY * 2, {
        signer: alice,
      });

      // Should revert with InsufficientBalance
      expect(result.isError).to.be.true;
    });

    it('should emit Transfer event', async () => {
      const result = await contract.tx.transfer(bob.address, 100, {
        signer: alice,
      });

      expectEvent(result, 'contracts', 'ContractEmitted');
      // In a real implementation, we'd decode the event data
      // and verify it contains the correct from, to, and value
    });
  });

  describe('Approval', () => {
    it('should approve spender', async () => {
      const result = await contract.tx.approve(bob.address, 500, {
        signer: alice,
      });

      expectSuccess(result);

      const allowance = await contract.query.allowance(alice.address, bob.address);
      expect(allowance.output.toNumber()).to.equal(500);
    });

    it('should emit Approval event', async () => {
      const result = await contract.tx.approve(bob.address, 300, {
        signer: alice,
      });

      expectEvent(result, 'contracts', 'ContractEmitted');
    });
  });

  describe('TransferFrom', () => {
    beforeEach(async () => {
      // Alice approves Bob to spend 1000 tokens
      await contract.tx.approve(bob.address, 1000, {
        signer: alice,
      });
    });

    it('should transfer tokens using allowance', async () => {
      const result = await contract.tx.transferFrom(
        alice.address,
        charlie.address,
        500,
        { signer: bob }
      );

      expectSuccess(result);

      const charlieBalance = await contract.query.balanceOf(charlie.address);
      expect(charlieBalance.output.toNumber()).to.equal(500);

      const remainingAllowance = await contract.query.allowance(
        alice.address,
        bob.address
      );
      expect(remainingAllowance.output.toNumber()).to.equal(500);
    });

    it('should fail when exceeding allowance', async () => {
      const result = await contract.tx.transferFrom(
        alice.address,
        charlie.address,
        2000,
        { signer: bob }
      );

      expect(result.isError).to.be.true;
    });
  });

  describe('Mint', () => {
    it('should mint tokens to account', async () => {
      const initialTotalSupply = await contract.query.totalSupply();

      const result = await contract.tx.mint(charlie.address, 5000, {
        signer: alice,
      });

      expectSuccess(result);

      const newTotalSupply = await contract.query.totalSupply();
      expect(newTotalSupply.output.toNumber()).to.equal(
        initialTotalSupply.output.toNumber() + 5000
      );

      const charlieBalance = await contract.query.balanceOf(charlie.address);
      expect(charlieBalance.output).to.be.gte(5000);
    });

    it('should fail when non-owner tries to mint', async () => {
      const result = await contract.tx.mint(charlie.address, 1000, {
        signer: bob,
      });

      expect(result.isError).to.be.true;
    });
  });

  describe('Burn', () => {
    it('should burn tokens from caller', async () => {
      const initialBalance = await contract.query.balanceOf(alice.address);
      const initialTotalSupply = await contract.query.totalSupply();

      const result = await contract.tx.burn(100, {
        signer: alice,
      });

      expectSuccess(result);

      const newBalance = await contract.query.balanceOf(alice.address);
      const newTotalSupply = await contract.query.totalSupply();

      expect(newBalance.output.toNumber()).to.equal(
        initialBalance.output.toNumber() - 100
      );
      expect(newTotalSupply.output.toNumber()).to.equal(
        initialTotalSupply.output.toNumber() - 100
      );
    });

    it('should fail when burning more than balance', async () => {
      const result = await contract.tx.burn(INITIAL_SUPPLY * 10, {
        signer: alice,
      });

      expect(result.isError).to.be.true;
    });
  });
});
