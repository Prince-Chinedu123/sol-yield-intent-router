# 🚀 SolYield Intent Router

A secure, non-custodial Solana vault protocol built with the **Anchor Framework**. This program serves as an "Intent Layer" that aggregates user SOL to be routed into high-yield LST strategies like JitoSOL.

## 🛡️ Security Architecture
- **Non-Custodial PDAs:** All user funds are held in Program Derived Addresses (PDAs), ensuring funds are controlled strictly by program logic, not admin private keys.
- **Admin-Only Staking:** Uses strict `require_keys_eq!` constraints to ensure only the designated authority can transition funds into a staking state.
- **Checked Arithmetic:** Implements Anchor's checked math to prevent overflow/underflow vulnerabilities during deposits.

## 🏗️ Technical Workflow
1. **Initialize:** Deploys a global state account using the `state_v3` seed to track vault metrics.
2. **Deposit:** Users transfer SOL into the vault PDA. The program updates the `total_locked` balance in the state account.
3. **Stake Intent:** The admin triggers the staking flag, signaling the liquidity is ready for LST routing.

## 🧪 Testing & Verification
The protocol is verified via a comprehensive TypeScript test suite (`tests/anchor.test.ts`). 
- **Deployment:** Successfully deployed to Solana Devnet.
- **Test Coverage:** 100% passing for the full lifecycle: `Initialize` -> `Deposit` -> `Stake` -> `State Validation`.

## ⚙️ How to Build
```bash
anchor build
anchor test
