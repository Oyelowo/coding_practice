whenever u define a storage variable, if u mark it as public keyboard, the contract will 
automatically create a public function with the same name
e.g.
message will already create a function name message that returns message value.
Essentially, exactly what getMessage does.

```ts
pragma solidity ^0.4.17;

contract Inbox {
    string public message; // stored on the blockchain
    
    function Inbox(string initialMessage) public {
        message = initialMessage;
    }
    
    function setMessage(string newMessage) public {
        message = newMessage;    // changes the blockchain
    }
    
    function getMessage() public view returns (string) {
        return message;  // doesn't change the blockchain.
    }
}

```



- When creating a contract transaction, the `to` field should be left blank.

- Anytime you want to change/update any data on the blockchain, you have to submit a transaction
  and wait for the transaction to be approved. This takes time on the public network, costs money and returns a hash.
  It may not appear on the test browser/network that the transaction takes time but in actual real-world case, it can take seconds.

- When you're sending a transaction(i.e changing data on blockchain), even if your function returns
  a value and has a return type of the value, you are not getting the value back. Rather, you will
  be getting a hash returned by the blockchain.

  ```ts
      // This function is not Instantaneous and you don't get the string value back, rather you get a hash back
      function setMessage(string newMessage) public returns (string) { // notice the return type here: the string will not be returned.
        message = newMessage;    // changes the blockchain
        return message;  // This will not be returned
    }

    ```

Sending a transaction to a function costs money
