import * as d3 from "d3";
import React, { StrictMode, useEffect, useState } from "react";
import { animated, useSpring } from "react-spring";
import { RecoilRoot } from "recoil";
import "./App.css";
import { useStuff } from "./recoil/Somethings";
import VoronoiHoverTracker from "./Voronoi/Voronoi";

const App = () => {
  return (
    <StrictMode>
      <RecoilRoot>
        <div

          className=""
          style={{
            display: "grid",
            justifyItems: "center",
            alignItems: "center",
          }}
        >
          <h1>this</h1>
          <VoronoiHoverTracker />
          {/*    <SpringPlay />
          <Somethings /> */}
          {/* <Gesture /> */}
          {/*       <Spring />
      <Chart /> */}
          <br />
          <br />
          <br />
          <br />
          <br />
          {/* <RandomCompo /> */}
          {/* <LineChart /> */}
        </div>
      </RecoilRoot>
    </StrictMode>
  );
};

export default App;

const SpringPlay = () => {
  const p = d3.line()([
    [10, 60],
    [40, 90],
    [60, 10],
    [190, 10],
  ]);
  const props = useSpring({
    testNumber: 1,
    from: { testNumber: 0 },
    config: { mass: 10, tension: 50, friction: 50, clamp: true },
  });
  console.log("fef", p);
  const stroke = useSpring({
    someX: 400,
    from: { someX: 0 },
    // config: { mass: 10, tension: 50, friction: 50, },
  });
  return (
    /*  <animated.div>
      {props.testNumber.interpolate((val) => val.toFixed(2))}

     
    </animated.div> */
    <animated.svg stroke="red" fill="none">
      <path
        d={String(p)}
        strokeDashoffset={stroke.someX.interpolate((val) => {
          console.log("uytre", val.toFixed(0));
          return val.toFixed(0);
        })}
        strokeDasharray={400}
      />
    </animated.svg>
  );
};

const RandomCompo = () => {
  const [value, setValue] = useState("");
  const { addName } = useStuff(value);

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) =>
    setValue(e.target.value);

  const addSome = () => {
    addName();
    setValue("");
  };

  useEffect(() => {
    const click = (e: KeyboardEvent) => {
      switch (e.keyCode) {
        case 13:
          addSome();
          break;
        default:
          break;
      }
    };
    document.addEventListener("keydown", click);
    return () => {
      document.removeEventListener("keydown", click);
    };
  });

  console.log("renders");
  return (
    <div>
      <input type="text" value={value} onChange={handleChange} name="" id="" />
      <button onClick={addSome}>Add name</button>
    </div>
  );
};

const useEvent = (event: DocumentEventMap) => {};
