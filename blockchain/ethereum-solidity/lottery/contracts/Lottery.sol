pragma solidity ^0.4.17;

contract Lottery {
    address public manager;
    address[] public players;

    function Lottery() public {
        manager = msg.sender; // msg is global object always available in our cod
    }

    function enter() public payable {
        require(msg.value > .01 ether); // Eth doesn't tell exactly why this fails if someone tries to send less ether. You can use the debugger tool for this

        players.push(msg.sender); // Get the address of the player - sender
    }

    function random_pseudo() private view returns (uint256) {
        return uint256(keccak256(block.difficulty, now, players)); // sha3 is a global function same with keccak256
    }

    function pickWinner() public restricted {
        // Now replaced with a restricted modifier
        // require(msg.sender == manager); // used to enforce security. i.e only manager can pick winner.

        uint256 index = random_pseudo() % players.length;
        players[index].transfer(this.balance); // Addresses are object e.g 0x0328982392......   | `this` is a reference to the current contract
        players = new address[](0); // reset the dynamic array and initialise it to empty array.
    }

    // used to prevent validation repetition
    modifier restricted() {
        require(msg.sender == manager);
        _; // the compiler runs the rest of the code of where restricted is used here
    }

    function getPlayers() public view returns (address[]) {
        return players;
    }
}
