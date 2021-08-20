// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

// import '@openzeppelin/contracts/math/SafeMath.sol';

contract Fallout {
  
  // using SafeMath for uint256;
  mapping (address => uint) allocations;
  address payable public owner;


  /* constructor */
  function Fal1out() public payable {
    owner = msg.sender;
    allocations[owner] = msg.value;
  }

  modifier onlyOwner {
	        require(
	            msg.sender == owner,
	            "caller is not the owner"
	        );
	        _;
	    }

  function allocate() public payable {
    // allocations[msg.sender] = allocations[msg.sender].add(msg.value);
    // Use the above instead. This is done for a demo purpose
    allocations[msg.sender] = allocations[msg.sender] + (msg.value);
  }

  function sendAllocation(address payable allocator) public {
    require(allocations[allocator] > 0);
    allocator.transfer(allocations[allocator]);
  }

  function collectAllocations() public onlyOwner {
    msg.sender.transfer(address(this).balance);
  }

  function allocatorBalance(address allocator) public view returns (uint) {
    return allocations[allocator];
  }
}


/* 
Solution
The constructor was mispelled in the contract 
which makes it callable as it is a public function.
Always check the contract for the correct spelling.
Better yet, use the keyword - constructor instead. 
await contract.Fal1out()
 */
