import type { NextPage } from "next";
import Link from "next/link";
import React from "react";
import Web3 from "web3";

const Home: NextPage = () => {
  const we = new Web3(Web3.givenProvider ?? "ws://localhost:8545");
  return (
    <div>
      <button>
        <Link href="/login">Goto Login</Link>
      </button>
    </div>
  );
};

export default Home;
