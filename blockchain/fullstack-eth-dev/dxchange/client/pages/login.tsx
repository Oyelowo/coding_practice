import React from "react";
import Web3 from "web3";
const web3 = new Web3();
const Login = () => {
  const handleClick = async () => {
    // --snip--
    const publicAddress = await web3.eth.getCoinbase();

    fetch(`users?publicAddress=${publicAddress}`)
      .then((response) => response.json())
      // If yes, retrieve it. If no, create it.
      .then((users) => (users.length ? users[0] : handleSignup(publicAddress)))
      // Popup MetaMask confirmation modal to sign message
      .then(handleSignMessage)
      // Send signature to back end on the /auth route
      .then(handleAuthenticate);
    // --snip--
  };

  const handleSignup = (publicAddress: string) =>
    fetch(`${process.env.REACT_APP_BACKEND_URL}/users`, {
      body: JSON.stringify({ publicAddress }),
      headers: {
        "Content-Type": "application/json",
      },
      method: "POST",
    }).then((response) => response.json());

  const handleSignMessage = async ({ publicAddress, nonce }: any) => {
    const rr = await web3.eth.personal.sign(
      web3.utils.utf8ToHex(`I am signing my one-time nonce: ${nonce}`),
      publicAddress,
 ""
    );
  };

  const handleAuthenticate = ({ publicAddress, signature }: any) =>
    fetch(`${process.env.REACT_APP_BACKEND_URL}/auth`, {
      body: JSON.stringify({ publicAddress, signature }),
      headers: {
        "Content-Type": "application/json",
      },
      method: "POST",
    }).then((response) => response.json());
  return (
    <div>
      Login page
      <hr />
      <br />
      Sign Message
    </div>
  );
};

export default Login;
