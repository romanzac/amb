# Simple Arbitrary Message Bridge for Counter SC

Relaying increments of a counter between two EVM blockchains. Example is using Mumbai -> Mumbai configuration.

### Prerequisites:
- NodeJS v16+   
- Truffle v5.6.5   
- Two wallets, for AMB role and ADMIN role  
  
Automatic relay:  
- Rust v1.65+   
- One more wallet for DAPP role   
  
### Deploy Smart Contracts:
cd amb   
edit env.example and rename to .env   
npm run deploy  
Make a note of contract addresses  

### Run Tests:  
npm run test

### Build Relayer:  
npm run compile (if not done in previous steps)   
npm run build      
Edit relayer/env.relayer.example rename to relayer/.env.relayer   

### Start Relayer Process:    
npm run relayer  

### Increment Counter N1:    
open new shell console     
npm run increment   
>Magic should happen at N2!  

