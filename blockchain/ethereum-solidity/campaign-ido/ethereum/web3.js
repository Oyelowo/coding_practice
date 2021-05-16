import Web3 from "web3";

/* interface EthWindow extends Window {
  ethereum: any;
}

declare let window: EthWindow;
 */
let web3;

if (typeof window !== "undefined" && typeof window.ethereum !== "undefined") {
  // We are in the browser and metamask is running.
  window.ethereum.request({ method: "eth_requestAccounts" });
  web3 = new Web3(window.ethereum);
} else {
  // We are on the server *OR* the user is not running metamask
  const provider = new Web3.providers.HttpProvider(
    "https://rinkeby.infura.io/v3/09b8eb45ff384275816832c284cd01c4"
  );
  web3 = new Web3(provider);
}

export default web3;
