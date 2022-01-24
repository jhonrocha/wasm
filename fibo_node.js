function fibo (num) {
  if (num <= 1) return num
  let resp = [0, 1]
  for (let i = 1; i < num; i++) {
    resp = [resp[1], resp[0] + resp[1]]
  }
  return resp[1]
}

console.log(fibo(BigInt(10000)))
