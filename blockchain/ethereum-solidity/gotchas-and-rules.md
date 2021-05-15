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
  It may not appear on the test browser/network(development environment) that the transaction takes time but in actual real-world case, it can take seconds.

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

- When sending a transaction, there is some amount of gas attached to it.


### GAS PRICE

- gasPrice: amount of Wei the sending is willing per unit gas to get the transaction processed.
- startGas/gasLimit: units of gas that this transaction can consume. i.e the total amount one is willing to pay.

e.g For transaction to call the function `doMath`:
gasPrice = 300;
gasLimit = 10;

```ts
    function doMath(int a, int b){
        a + b;    ---> costs 3 gas
        b - a;    ---> costs 3 gas // Transaction halts here and no other code is executed within the function. because 3 + 3 + 5 = 11 which is more than our gas limit- 10
        a * b;    ---> costs 5 gas
        a == b;   ---> costs 3 gas
                Total needed: 14 gas
    }
```

In this case, the transaction halts here and no other code is executed within the function. because 3 + 3 + 5 = 11 which is more than our gas limit- 10.
In this case, we can increase the gas limit to 20, which then makes the entire transaction run as it will now be greater than 14.

Total cost = 300wei/gas * 14 gas = 4,200 wei. 

Max possible cost(if gas Limit is increased to 20) = 300wei/gas * 20 gas = 6,000 wei

NB: you spend gas to modify and store data on the network.
Whoever is creating a contract has to pay the gas fee.  This then starts to affect the cost of using our application.
e.g imagine people have to pay to create a post on linkedin, this starts to affect the cost of using linkedin which may prevent users.
This limits the type of application you can build on ethereum but there are ways around this.



When working with an array, you have to access an individual element by providing the index.

As at now(2018-2021), In solidity, you can build nested dynamic array, but not yet supported in the ABI/JS/Web3 bridge.
Since string is stored as a dynamic array, it means we cannot have array of string.

e.g `["red", "green", "blue"]`
Here, the strings themselves are stored as dynamic array, thus with the above caveat, they cannot yet be stored
in an array, which will make the entire thing a nested array
