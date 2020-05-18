import React from "react";
import "./App.css";
import Chart from "./Chart";
import Spring from "./Spring";

function App() {
  return (
    <div
      className=""
      style={{
        marginTop: 300,
        display: "grid",
        justifyItems: "center",
        alignItems: "center",
      }}
    >
      <Spring />
      <Chart />
    </div>
  );
}

export default App;
