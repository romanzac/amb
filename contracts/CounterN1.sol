// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

contract CounterN1 {

    constructor(address payable amb) {
        owner = msg.sender;
        AMB = amb;
        counter = 0;
    }

    address owner;                             // Contract admin
    address payable AMB;                       // Trusted relayer's address
    address receivingCounter;                  // Receiving smart contract address
    uint256 counter;                           // Local counter value

    uint256 RECEIVER_BASE_FEE = 5000000 wei;   // Average GAS limit for the destination network

    mapping (address => uint256) targetValue;  // A place to leave message for AMB acting on behalf of receivingCounter

    // Post deployment call for setting counter party's address
    function setReceivingCounter(address receiver) public {
        require(msg.sender == owner);
        receivingCounter = receiver;
    }

    // Any DApp which pays fees for AMB could send counter's value to the receiver
    function send() public payable {
        uint256 amount = msg.value;
        require(amount >= RECEIVER_BASE_FEE, "Minimum fee has not been attached.");
        targetValue[receivingCounter] = counter;
        AMB.transfer(amount);
    }

    // Counter could be synchronized either by AMB or incremented by 1 by any DApp
    function increment(address receiver, uint256 targetVal) public {
        if (receiver == address(this)) {
            require(msg.sender == AMB, "Only trusted relayer can synchronize counter.");
            if (counter < targetVal) {
                counter = targetVal;
            }
        } else {
            counter++;
        }
    }

    function getTargetValue(address receiver) public view returns (uint256) {
        return targetValue[receiver];
    }

    function getCounter() public view returns (uint256) {
        return counter;
    }
}


