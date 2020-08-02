import * as d3 from "d3";
import { Delaunay } from "d3-delaunay";
import React, { useState } from "react";

interface Datum {
  category: string;
  x: number;
  y: number;
}

const data: Datum[] = [
  { category: "cold", x: 14.2, y: 215 },
  { category: "cold", x: 16.4, y: 325 },
  { category: "cold", x: 11.9, y: 185 },
  { category: "cold", x: 15.2, y: 332 },
  { category: "cold", x: 18.5, y: 406 },
  { category: "cold", x: 22.1, y: 522 },
  { category: "cold", x: 19.4, y: 412 },
  { category: "cold", x: 25.1, y: 614 },
  { category: "cold", x: 23.4, y: 544 },
  { category: "cold", x: 18.1, y: 421 },
  { category: "cold", x: 22.6, y: 445 },
  { category: "cold", x: 17.2, y: 408 },
];

const height = 700;
const width = 700;

const VoronoiHoverTracker = () => {
  const [hoveredDatum, setHoveredDatum] = useState<Datum | null>(null);
  // const [hoveredX, setHoveredX] = useState<number | null>(null);
  // const [hoveredY, setHoveredY] = useState<number | null>(null);
  const yScale = d3
    .scaleLinear()
    .domain(d3.extent(data, (d) => d.y) as [number, number])
    .range([height, 0]);

  const xScale = d3
    .scaleLinear()
    .domain(d3.extent(data, (d) => d.x) as [number, number])
    .range([0, width]);

  const points = data.map(({ x, y }) => [xScale(x), yScale(y)]);
  const delaunay = Delaunay.from(points);
  const voronoi = delaunay.voronoi([0, 0, width - 0.5, height - 0.5]);

  console.log(hoveredDatum);
  //const handleHovered = useMemo(() => (v: Datum) => setHovered(v), []);
  return (
    <svg
      width={width}
      height={height}
      style={{ border: "1px solid red" }}
      pointerEvents="none"
    >
      <g style={{ border: "1px solid red" }}>
        <text
          x={hoveredDatum?.x ? 2 + xScale(hoveredDatum.x) : undefined}
          y={hoveredDatum?.y ? 6 + yScale(hoveredDatum.y) : undefined}
          fontWeight="100"
          stroke="red"
          fontSize="12"
        >
          {hoveredDatum?.x}
          {hoveredDatum?.y}
          {hoveredDatum?.category}
        </text>
        {data.map(({ x, y }, i) => {
          return (
            <g key={i} pointerEvents="none">
              <text
                x={xScale(x)}
                y={yScale(y)}
                fontWeight="100"
                stroke="#bbb"
                fontSize="12"
              >
                {x}
                {y}
              </text>
              <circle
                cx={xScale(x)}
                cy={yScale(y)}
                r={3}
                stroke="red"
                pointerEvents="none"
              />

              {data.map(({ x, y, category }, i) => (
                <path
                  d={voronoi.renderCell(i)}
                  stroke="#fff"
                  strokeWidth="2"
                  fill={`none`}
                  onMouseOver={() => {
                    setHoveredDatum({ x, y, category });
                  }}
                  onMouseLeave={() => {
                    setHoveredDatum((currentDatum) =>
                      currentDatum === { x, y, category } ? null : currentDatum
                    );
                  }}
                  pointerEvents="visibleStroke"
                  onClick={() => console.log("erer")}
                />
              ))}
            </g>
          );
        })}
        <path
          pointerEvents="none"
          d={voronoi.render()}
          stroke="#eee"
          strokeWidth="2"
          fill="none"
        />

        <path
          pointerEvents="none"
          d={voronoi.renderBounds()}
          stroke="#eaeaea"
          strokeWidth="2"
          fill="none"
        />
      </g>
    </svg>
  );
};

export default VoronoiHoverTracker;

/* {data.map(({ x, y, category }, i) => (
  <polygon
    points={voronoi
      .cellPolygon(i)
      .map(([x, y]) => `${x}, ${y}`)
      .join(" ")}
    stroke="#fff"
    strokeWidth="2"
    fill="none"
    onMouseEnter={() => {
      setHoveredDatum({ x, y, category });
    }}
    onMouseLeave={() => {
      setHoveredDatum((currentDatum) =>
        currentDatum === {x,y,category} ? null : currentDatum
      );
    }}
    pointerEvents="visibleStroke"
    onClick={() => console.log("erer")}
  />
))} */
