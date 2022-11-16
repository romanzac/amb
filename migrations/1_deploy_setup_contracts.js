var CounterN1 = artifacts.require("CounterN1");
var CounterN2 = artifacts.require("CounterN2");

const AMB_ADDRESS = process.env.AMB_ADDRESS;
/**
 * Deploy N1 and N2 Contract instance
 */
module.exports = async function (deployer) {
  console.log("Deploying N1 Counter 👋");
  await deployer.deploy(CounterN1, AMB_ADDRESS);

  console.log("Deploying N2 Counter 👋👋");
  await deployer.deploy(CounterN2, AMB_ADDRESS);

  const instanceN1 = await CounterN1.deployed();
  const instanceN2 = await CounterN2.deployed();

  const tx2 = await instanceN2.setReceivingCounter(instanceN1.address);
  console.log(`🙌 Setup for Counter instance N2 finished! ${tx2.receipt.transactionHash}`);

  const tx = await instanceN1.setReceivingCounter(instanceN2.address);
  console.log(`🙌 Setup for Counter instance N1 finished! ${tx.receipt.transactionHash}`);

};

