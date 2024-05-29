const generate_prime_list = (max_size : number) : number[] => {
  const p : number = Math.ceil(Math.sqrt(max_size + 1));
  const primes : number[] = [];

  const capacity : number = Math.pow(p, 2);
  let array : number[] = [];
  for (let i = 2; i < capacity; i++) {
    array.push(i);
  }

  while (true) {
    let prime : number = array.shift()!;
    primes.push(prime);
    array = array.filter((value) => value % prime !== 0);
    if (prime > p) {
      array.forEach((v) => primes.push(v));
      break;
    }
  }

  return primes;
}


const main = (s: number): number => {
  const primes: number[] = generate_prime_list(s);
  let ret : number = 2;

  for (const prime of primes) {
    if (prime >= s) {
      ret = prime;
      break;
    }
  }

  return ret;
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
