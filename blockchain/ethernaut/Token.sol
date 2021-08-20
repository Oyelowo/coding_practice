

/* 
Try to transfer 21 tokens to anyone but yourself; since balances are unsigned integers, the balance of each account will always be positive. Therefore, 20-21 = 2^256 - 1. That's very large.
 */


// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

contract Token {

  mapping(address => uint) balances;
  uint public totalSupply;

  constructor(uint _initialSupply) public {
    balances[msg.sender] = totalSupply = _initialSupply;
  }

  function transfer(address _to, uint _value) public returns (bool) {
    // 20-21 = 2^256 - 1. So, this passes cos the balances are unsigned integers.
    require(balances[msg.sender] - _value >= 0);
    // 20-21 = 2^256 - 1
    balances[msg.sender] -= _value;
    // use another address as the recipient
    balances[_to] += _value;
    return true;
  }

  function balanceOf(address _owner) public view returns (uint balance) {
    return balances[_owner];
  }
}
