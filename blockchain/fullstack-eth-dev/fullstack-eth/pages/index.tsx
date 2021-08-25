import type { NextPage } from "next";
import Greetings from "../components/Greetings";
import dynamic from "next/dynamic";

const GreetingsWithNoSSR = dynamic(() => import("../components/Greetings"), {
  ssr: false,
});


const Home: NextPage = () => {
  return (
    <div>
      <GreetingsWithNoSSR />
    </div>
  );
};

export default Home;
