import { configure, processCLIArgs, run } from '@japa/runner'
import { assert } from '@japa/assert'
import { expect } from '@japa/expect'

processCLIArgs(process.argv.splice(2))
configure({
  files: ['tests/js/*.test.js'],
  plugins: [
    expect(),
    assert(),
  ],
})

run()