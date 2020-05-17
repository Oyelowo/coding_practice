import { max, min } from "d3-array";
import { scaleLinear, scaleTime } from "d3-scale";
import * as d3 from "d3-shape";
import React, { FC } from "react";

interface Datum {
  date: Date;
  value: number;
}

const margin = {
  left: 15,
  right: 15,
  bottom: 15,
  top: 15,
};

const padding = {
  left: 5,
  right: 5,
  bottom: 5,
  top: 5,
};

const width = 600 - margin.left - margin.right;
const height = 200 - margin.top - margin.bottom;

const Chart: FC = ({ children }) => {
  const data = [
    { date: new Date(2007, 3, 24), value: 93.24 },
    { date: new Date(2007, 3, 25), value: 95.35 },
    { date: new Date(2007, 3, 26), value: 98.84 },
    { date: new Date(2007, 3, 27), value: 99.92 },
    { date: new Date(2007, 3, 30), value: 99.8 },
    { date: new Date(2007, 4, 1), value: 99.47 },
  ];
  
  const minX = min(data, (d) => d.date) as Date;
  const maxX = max(data, (d) => d.date) as Date;


  const minY = min(data, (d) => d.value) as number;
  const maxY = max(data, (d) => d.value) as number;

  const scaleX = scaleTime()
    .domain([minX, maxX])
    .range([margin.left, width])
    .nice();
  const scaleY = scaleLinear().domain([minY, maxY]).range([0, height]);

  const line3 = d3
    .line<Datum>()
    .x((d) => scaleX(d.date))
    .y((d) => scaleY(d.value));

  console.log(line3(data));

  const p = d3
    .line<{ date: number; value: number }>()
    .x((d) => scaleX2(d.date))
    .y((d) => scaleY(d.value));

  return (
    <div>
      <g transform={`translate(${margin.left}, ${margin.top})`}>
        {" "}
        <svg
          style={{ background: "#eaeaea" }}
          width={width}
          height={height}
          fill="none"
          stroke="green"
        >
          <path d={line3(data)!} />
          {data.map((d) => (
            <circle
              cy={scaleY(d.value)}
              cx={scaleX(d.date)}
              r="3"
              fill="black"
            />
          ))}
        </svg>
      </g>
    </div>
  );
};

export default Chart;
