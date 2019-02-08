
## Hello and welcome to JellySchema, maintainer !

This talks about how to run this project locally and how to make changes to it.
 
You can just run `./scripts/install.sh` to get all dependencies going. This will install Rust and Node via NVM - please review the script before running, to see if this is what you want on your machine.

To make sure everything is working run `./scripts/test.sh` - this will launch all the Rust tests and then package npm package and run node tests on it.

## Tests

Most of the testing is example-based, with examples stored in `tests/data`.
This data is used to generate regular unit tests during the build process.
There are 2 levels of subdirectories there.
The first level indicates a test group name - this will be translated into a test module name and then there is a directory inside of that - that would indicate a unit test name.
The test directories each can contain:

Either:

```
input-schema.yml
output-json-schema.json
output-uischema.json
```

or:

```
input-schema.yml
output-error
```

Meaning of each file by its name:
* `input-schema.yml` - this is jellyschema input
* `output-json-schema.json` - this is JSONSchema that should be produced from that input jellyschema
* `output-uischema.json` - this is UISchema that should be produced from that input jellyschema
* `output-error` - an empty marker file signalling that the input-schema should produce error when we're trying to generate output schemas from it

Note:
We expect the code to never panic so the test with `output-error` checks for `Err` present, but if it panics instead it's gonna be marked as failed.

## Fuzzing
Fuzz tests are not included in the CI runs as they typically take a long time to run and the timing is quite unpredictable.  
We're using pretty standard [cargo fuzz] configuration with seeds here, based on [libFuzzer].  
To run the default fuzzer configuration do `./fuzz/run-fuzzer.sh`. 
Warning: running the fuzzer via this script will set your rustup environment override to nightly.
 
This will use seed jellyschema files from `fuzz/seeds` to jumpstart the process.  
You can cancel the fuzzing at any time by pressing `ctrl-c`. If you then run the fuzzer again, it will start where it left off - the temporary state is saved to `fuzz/corpus`  

### Fuzzing targets
It is only possible to fuzz one target at the time, the default target is `any_input` - that tests whether random input will crash the JellySchema (make it panic).
TODO: add a fuzzing target where we make sure there is no `Err` when the yaml validates as jellyschema.

[cargo fuzz]: https://fuzz.rs/book/cargo-fuzz/guide.html
[libFuzzer]: https://llvm.org/docs/LibFuzzer.html
