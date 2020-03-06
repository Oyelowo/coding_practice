const express = require("express");

const app = express();

app.get("/", (req, res) => {
  res.send("The way you code!");
});

app.listen(8080, () => {
  console.log("listening in port - 8080");
});
