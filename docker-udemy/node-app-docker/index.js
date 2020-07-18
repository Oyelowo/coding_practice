const express = require("express");

const app = express();

app.get("/", (req, res) => {
  res.send("Another day, another code!");
});

const PORT = 8080
app.listen(PORT, () => {
  console.log(`listening in port - ${PORT}`);
});
