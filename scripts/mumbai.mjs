#!/user/bin/env zx

// 1. Deploy and setup on the same network
await $`truffle migrate --network=mumbai --f 1 --to 1 --skip-dry-run`;
