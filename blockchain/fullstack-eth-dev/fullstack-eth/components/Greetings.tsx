import { ethers } from "ethers";
import type { NextPage } from "next";
import React, { useState } from "react";
import Greeter from "../artifacts/contracts/Greeter.sol/Greeter.json";
import Token from "../artifacts/contracts/Token.sol/Token.json";

// Update with the contract address logged out to the CLI when it was deployed
const greeterAddress = "0x4CDcE4d6c1fdE265cAC1b796cbEf988669C584Bd";
const tokenAddress = "0xf81e5AF34bAd029D3f72C177416AD751Cae22eAa";

/* 
Greeter deployed to: 0x4CDcE4d6c1fdE265cAC1b796cbEf988669C584Bd
Token deployed to: 0xf81e5AF34bAd029D3f72C177416AD751Cae22eAa
*/

const Greetings: NextPage = () => {
  //console.log("Greeter ABI:", Greeter.abi);
  const [userAccount, setUserAccount] = useState('');
  const [amount, setAmount] = useState('');
  const ethereum = (window as any).ethereum;
  const [greeting, setGreeting] = useState("");
  // request access to the user's MetaMask account

  const requestAccount = async () => {
    await ethereum.request({ method: "eth_requestAccounts" });
  };

  // call the smart contract, read the current greeting value
  const fetchGreeting = async () => {
    // if (typeof ethereum !== "undefined" && ethereum.isConnected()) {
    if (typeof ethereum !== undefined) {
      const provider = new ethers.providers.Web3Provider(ethereum);
      const contract = new ethers.Contract(
        greeterAddress,
        Greeter.abi,
        provider
      );
      console.log("contract: ", await contract);
      try {
        const data = await contract.greet();
        console.log("data: ", data);
      } catch (error) {
        console.log("Error: ", error);
      }
    }
  };

  async function getBalance() {
    if (typeof ethereum !== undefined) {
      const [account] = await ethereum.request({
        method: "eth_requestAccounts",
      });
      const provider = new ethers.providers.Web3Provider(ethereum);
      const contract = new ethers.Contract(tokenAddress, Token.abi, provider);
      const balance = await contract.balanceOf(account);
      console.log("Balance: ", balance.toString());
    }
  }

  // call the smart contract, send an update
  const setGreetingFn = async () => {
    if (!greeting) return;
    if (typeof ethereum !== undefined) {
      await requestAccount();
      const provider = new ethers.providers.Web3Provider(ethereum);
      const signer = provider.getSigner();
      console.log("signer: ", await signer.getAddress());
      const contract = new ethers.Contract(greeterAddress, Greeter.abi, signer);
      const transaction = await contract.setGreeting(greeting);
      await transaction.wait();
      fetchGreeting();
    }
  };

  async function sendCoins() {
    if (typeof ethereum !== undefined) {
      await requestAccount();
      const provider = new ethers.providers.Web3Provider(ethereum);
      const signer = provider.getSigner();
      const contract = new ethers.Contract(tokenAddress, Token.abi, signer);
      const transaction = await contract.transfer(userAccount, amount);
      await transaction.wait();
      console.log(`${amount} Coins successfully sent to ${userAccount}`);
    }
  }

  return (
    <div>
      {" "}
      <header className="App-header">
        <button onClick={fetchGreeting}>Fetch Greeting</button>
        <button onClick={setGreetingFn}>Set Greeting</button>
        <input
          onChange={(e) => setGreeting(e.target.value)}
          placeholder="Set greeting"
        />
        <br />
        Greeting: {greeting}
      </header>
      <br />
      <button onClick={getBalance}>Get Balance</button>
      <button onClick={sendCoins}>Send Coins</button>
      <input
        onChange={(e) => setUserAccount(e.target.value)}
        placeholder="Account ID"
      />
      <input onChange={(e) => setAmount(e.target.value)} placeholder="Amount" />
    </div>
  );
};

export default Greetings;
