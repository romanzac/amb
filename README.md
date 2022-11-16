# Arbitrary Message Bridge for Counter SC

### Prerequisites:
- NodeJS v16+   
- Truffle v5.6.5   
- Two wallets, for AMB role and ADMIN role  
  
Optional for automatic relay:  
- Rust v1.65+   
- One more wallet for DAPP role   
  
Everything is so far pre-configured for Polygon testnet (Mumbai)  

### Deploy Smart Contracts:
cd amb   
edit env.example and rename to .env   
npm run deploy  
Make a note of contract addresses  

### Run Tests:  
npm run test

### Build Relayer (optional):  
npm run build      
Edit relayer/env.relayer.example rename to relayer/.env.relayer   

### Run Relayer (optional):    
npm run relayer  

### Increment Counter N1 (optional and occasional :):    
open different shell console     
npm run increment   
>Magic should happen!  

