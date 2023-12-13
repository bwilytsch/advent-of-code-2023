const fs = require("fs");

function main() {
  const file = fs.readFileSync("./test.txt", "utf8").split("\n\n");

  const seeds = file
    .shift()
    .split(": ")[1]
    .split(" ")
    .map((x) => +x);

  const maps = file.reduce((acc, cur) => {
    const [label, entries] = cur.split(":");

    console.log(label.split(" ")[0], entries);

    return acc;
  }, []);

  console.log(JSON.stringify(maps, null, 4));
}

main();
