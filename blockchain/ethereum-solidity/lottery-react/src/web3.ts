import Web3 from "web3";

interface EthWindow extends Window {
  ethereum: any;
}

declare let window: EthWindow;

window.ethereum.request({ method: "eth_requestAccounts" }); // from metamask that is connected to the rinkeby netwoek

const web3 = new Web3(window.ethereum);

export default web3;
