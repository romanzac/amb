var CounterN1 = artifacts.require("CounterN1");
var CounterN2 = artifacts.require("CounterN2");

const ADMIN_ADDRESS = process.env.ADMIN_ADDRESS;
const AMB_ADDRESS = process.env.AMB_ADDRESS;

describe("Counter", function () {
    it("Should return incremented counter value after local increment call", async function () {
        this.timeout(0);
        const instN1 = await CounterN1.deployed();

        const before = await instN1.getCounter.call();
        assert.equal(before.toNumber(), 0);

        // Increment counter N1 locally
        const incTx = await instN1.increment(ADMIN_ADDRESS, 0);

        const after = await instN1.getCounter.call();
        assert.equal(after.toNumber(), 1);
    });

    it("Should run main workflow: increment N1 -> send -> increment N2", async function () {
        this.timeout(0);
        const instN1 = await CounterN1.deployed();
        const instN2 = await CounterN2.deployed();

        const before = await instN1.getCounter.call();
        assert.equal(before.toNumber(), 1);

        // Increment counter N1 locally
        await instN1.increment(ADMIN_ADDRESS, 0);

        // Send N1's counter value to N2 a pay for the fees
        await instN1.send({
            from: ADMIN_ADDRESS,
            value: 5000000,
        });

        // Pickup value for N2
        const targetVal = await instN1.getTargetValue(instN2.address);
        assert.equal(targetVal.toNumber(), 2);

        const beforeReceive = await instN2.getCounter.call();
        assert.equal(beforeReceive.toNumber(), 0);

        // Synchronize value with N2
        await instN2.increment(instN2.address, targetVal, {from: AMB_ADDRESS});

        const afterReceive = await instN2.getCounter.call();
        assert.equal(afterReceive.toNumber(), 2);

    });

});