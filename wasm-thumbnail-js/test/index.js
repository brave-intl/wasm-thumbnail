import { resize_and_pad } from "wasm-thumbnail-js";
import fs from "fs";

console.log("hello");

if (process.argv.length !== 4) {
    console.error("usage: test infile outfile");
    process.exit(-1);
}

const [, , infile, outfile] = process.argv;
const buffer = fs.readFileSync(infile);

console.log("converting...");
const result = resize_and_pad(buffer, 920, 750, 500000);
console.log(result);

fs.writeFileSync(outfile, result);