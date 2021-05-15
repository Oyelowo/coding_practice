pragma solidity ^0.4.17;

contract Campaign {
    struct Request {
        string description;
        uint value;
        address recipient;
        bool complete;
        uint approvalCount;  // This is stored separately because mapping does not store keys and we wouldn't be able to do e.g Object.keys(mapping).length
        mapping(address => bool ) approvals;
    }
    
    Request[] public requests;
    address public manager;
    uint public minimumContribution;
    mapping(address => bool) public approvers;
    uint approversCount;


    
    modifier restrictToManager {
         require(msg.sender == manager);
         _;
    }
    
    
    function Campaign(uint minimum)public {
        manager = msg.sender;
        minimumContribution = minimum;
    }
    
    function contribute() public payable {
        require(msg.value > minimumContribution);
        
        approvers[msg.sender] = true;  // msg.sender does not get stored in approvers map, only the value true is stored.
        approversCount++;
    }
    
    function createRequest(string description, uint value, address recipient) public restrictToManager {
        require(approvers[msg.sender]);
        
        Request memory newRequest = Request ({
            description: description,
            value: value,
            recipient: recipient,
            complete: false,
            approvalCount: 0
            // we dont have to initiallise a reference type. Here the mapping
        });
        
        requests.push(newRequest);
       
    }
    
    function approveRequest(uint index) public {
        Request storage request = requests[index];  // we use storage because we actually want to mutate the data in the storage on the network.
        
        require(approvers[msg.sender]);
        require(!request.approvals[msg.sender]);
        
        request.approvals[msg.sender] = true;
        request.approvalCount++;
        
    }
    
    
    function finalizeRequest(uint index) public restrictToManager {
        Request storage request = requests[index];
        require(!request.complete);
        require(request.approvalCount > (approversCount/2));
        
        
        request.recipient.transfer(request.value);
        request.complete = true;
    }
    

    
}
