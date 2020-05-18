import React, { useState } from "react";
import { animated, useSpring } from "react-spring";
const Spring = () => {
  const [isIn, setIsIn] = useState(true);
  const goOut = isIn ? 1 : 0;
  const goIn = isIn ? 0 : 1;

  const goOut2 = isIn ? 270 : 0;
  const goIn2 = isIn ? 0 : 270;

  const props = useSpring({ opacity: goOut, from: { opacity: goIn } });
  const props2 = useSpring({ x: goOut2, from: { x: goIn2 } });
  const toggle = () => setIsIn((is) => !is);
  return (
    <div>
      <animated.div style={props}>
        <p>I will fade in</p>
      </animated.div>
      <animated.svg
        strokeDashoffset={props2.x}
        stroke-dasharray="270"
        stroke="red"
        fill="white"
      >
        <path
          fill="none"
          stroke="red"
          d="M 10,30
       A 20,20 0,0,1 50,30
       A 20,20 0,0,1 90,30
       Q 90,60 50,90
       Q 10,60 10,30 z"
        />
      </animated.svg>
      <button onClick={toggle}>Toggle</button>
    </div>
  );
};

export default Spring;
