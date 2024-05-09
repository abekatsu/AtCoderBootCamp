const main = (inputs: string) => {
  const [n, m, x, ...a] = inputs.split(/\s/).map(Number);

  let l_toll_gate = 0;
  let r_toll_gate = 0;
  a.forEach( (value: number) => {
    if (value < x) {
      l_toll_gate += 1;
    } else if (value > x) {
      r_toll_gate += 1;
    }
  });

  return Math.min(l_toll_gate, r_toll_gate);
}

(async () => {
  const decoder = new TextDecoder();
  let myinputs: string = "";
  for await (const chunk of Deno.stdin.readable) {
    myinputs = myinputs + decoder.decode(chunk).replace(/(\r\n|\n|\r)/gm, " ");
  }
  console.log(main(myinputs.trim())); // necessary trimming last space.
})();
