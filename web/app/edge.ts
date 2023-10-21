
const argon2 = require('argon2-browser/dist/argon2-bundled.min.js')


export default async function add(num1: number) {
  argon2.hash({ pass: '123456', salt: 'asdfasdf', time: 3, mem: 4096, parallelism: 1,
    hashLen: 32, type: argon2.ArgonType.Argon2id}).then((hash: any) => {
    console.log(hash)
  })
  return num1 + 1
}

export const config = { runtime: 'edge' }
