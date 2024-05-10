const main = (s: number): number => {
  const list: number[] = Array.from([s]);

  while (true) {
    const prev_a : number = list.at(list.length - 1) as number;
    let next_a = 0;

    if (prev_a % 2 == 0) {
      next_a = prev_a / 2;
    } else {
      next_a = prev_a * 3 + 1;
    }

    const last_find = list.find((element: number) => element == next_a);
    if (last_find) {
      break;
    } else {
      list.push(next_a);
    }
  }

  return list.length + 1;
}

(async () => {
  const decoder = new TextDecoder();
  let inputs: string = "";
  for await (const chunk of Deno.stdin.readable) {
    inputs += decoder.decode(chunk).replace(/(\r\n|\n|\r)/gm, " ");
  }

  let s: number = Number(inputs.trim());
  console.log(main(s)); // necessary trimming last space.
})();
