async function main() {
  console.log("run1");
  await a1();
  await a2();
  console.log("run2");
  console.log("run3");
}

async function a1() {
  console.log("a1");
}
async function a2() {
  console.log("a2");
}

main();
