import * as d3 from "d3";
import { eachDayOfInterval } from "date-fns";
import React from "react";
import styled from 'styled-components';


const Tooltip = styled.foreignObject({
  x: 3,
})

const data = [
  {
    score: 62,
    date: "2020-02-29",
  },
  {
    score: 73,
    date: "2020-03-01",
  },
  {
    score: 28,
    date: "2020-03-02",
  },
  {
    score: 77,
    date: "2020-03-03",
  },
  {
    score: 50,
    date: "2020-03-04",
  },
  {
    score: 87,
    date: "2020-03-05",
  },
  {
    score: 66,
    date: "2020-03-06",
  },
  {
    score: 94,
    date: "2020-03-07",
  },
  {
    score: 63,
    date: "2020-03-08",
  },
];

type CoolDatum = typeof data[number];

const margins = {
  TOP: 130,
  RIGHT: 40,
  LEFT: 40,
  BOTTOM: 100,
};

const svgProps = {
  HEIGHT: 500,
  WIDTH: 700,
};

const chartAreaProps = {
  HEIGHT: svgProps.HEIGHT - (margins.TOP + margins.BOTTOM),
  WIDTH: svgProps.WIDTH - (margins.RIGHT + margins.LEFT),
};
const paddings = {
  TOP: 10,
  RIGHT: 10,
  LEFT: 10,
  BOTTOM: 10,
};

const result = eachDayOfInterval({
  start: new Date("2020-02-29"),
  end: new Date("2020-03-08"),
});

const LineChart = () => {
  const [minY, maxY] = d3.extent(data, (d) => d.score) as [number, number];
  const [minX, maxX] = [new Date("2020-02-29"), new Date("2020-03-08")];

  const yScale = d3
    .scaleLinear()
    .domain([0, maxY])
    .range([chartAreaProps.HEIGHT, 0]).nice();

  const xScale = d3
    .scaleTime()
    .domain([minX, maxX])
    .range([0, chartAreaProps.WIDTH]);

  const line = d3
    .line<CoolDatum>()
    .curve(d3.curveCardinal)
    .y((d) => yScale(d.score))
    .x((d) => xScale(new Date(d.date)));

  return (
    <svg
      width={svgProps.WIDTH}
      height={svgProps.HEIGHT}
      style={{ background: "", border: "1px solid #eff" }}
    >
      <g transform={`translate(${margins.LEFT}, ${margins.TOP})`}>
        {/* Char Area Bounding Box */}
        {/*         <rect
          width={chartAreaProps.WIDTH}
          height={chartAreaProps.HEIGHT}
          fill="none"
          stroke="purple"
        /> */}

        {/* Data Line */}
        <path
          d={line(data) ?? ""}
          fill="none"
          stroke="#8b95e1"
          strokeWidth="2.5"
        />

        {/* Data points */}
        {data.map((el) => {
          return (
            <circle
              cx={xScale(new Date(el.date))}
              cy={yScale(new Date(el.score))}
              r={5}
              fill="#fff"
              stroke="#8b95e1"
            />
          );
        })}

<foreignObject  />
        {/* Grid Lines */}
        {[0, 25, 50, 75, 100, 125].map((score) => {
          return (
            <>
              <line
                x1={0}
                x2={chartAreaProps.WIDTH}
                y1={yScale(score)}
                y2={yScale(score)}
                fill="#fff"
                stroke="#eaeaea"
                strokeOpacity="0.5"
              />

              {/* Y-axis */}
              <text
                x={5 - margins.LEFT}
                y={yScale(score)}
                fontWeight="100"
                stroke="#bbb"
                fontSize="12"
              >
                {score > 100 ? null : score}
              </text>
            </>
          );
        })}

        {/* X-Axis label */}
        <g transform={`translate(0, ${chartAreaProps.HEIGHT})`}>
          {result.map((date) => {
            return (
              <>
                <text
                  x={xScale(date)}
                  y={0}
                  strokeWidth="1"
                  fontSize="12"
                  fontWeight="100"
                  stroke="#bbb"
                  transform={`translate(0, ${30})`}
                >
                  {padDateWithZero(date.getDate().toLocaleString())}
                </text>
              </>
            );
          })}

          {/* Highlighted X-axis */}
          {[
            new Date("2020-02-29"),
            new Date("2020-03-01"),
            new Date("2020-03-06"),
            new Date("2020-03-08"),
          ].map((highlight, i) => {
            return (
              <>
                <line
                  x1={xScale(highlight) - 12}
                  x2={xScale(highlight) + 12}
                  stroke="red"
                />
                <text
                  x={xScale(highlight) - 9}
                  transform="translate(0, 50)"
                  stroke="#aaa"
                  fontWeight="100"
                >
                  {i === 0 ? "Feb" : i === 1 ? "Mar" : null}
                </text>
              </>
            );
          })}
        </g>
      </g>
    </svg>
  );
};

export default LineChart;

const padDateWithZero = (number: string) => {
  return number.length > 1 ? number : `0${number}`;
};
