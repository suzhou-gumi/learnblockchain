// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.30;

contract Bank {
    constructor() {
        owner = tx.origin;
    }

    address private owner;
    mapping(address => uint256) private balances;
    uint[3] private topThreeDepositors;

    receive() external payable {
        require(msg.value > 0, "Deposit must be greater than zero");
        balances[msg.sender] += msg.value;
        setTopThreeDepositors(msg.sender);
    }

    function withdraw(uint256 amount) public {
        require(msg.sender == owner, "Only owner can withdraw");
        require(
            address(this).balance >= amount,
            "Insufficient contract balance"
        );
        payable(owner).transfer(amount);
    }

    function getBalance() public view returns (uint256) {
        return balances[msg.sender];
    }

    function setTopThreeDepositors(address depositor) private {
        uint256 amount = balances[depositor];
        for (uint i = 0; i < topThreeDepositors.length; i++) {
            if (topThreeDepositors[i] < amount) {
                for (uint j = topThreeDepositors.length - 1; j > i; j--) {
                    topThreeDepositors[j] = topThreeDepositors[j - 1];
                }
                topThreeDepositors[i] = amount;
                break;
            }
        }
    }
}
