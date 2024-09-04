# VM that uses LC-3 architecture

This project is an implementation of a LC-3 virtual machine, following [this tutorial](https://www.jmeiners.com/lc3-vm/).

### How to run
- Clone the repository with `git clone`, and enter de directory containing the VM code.
- Run `make build` to compile the code.
- The VM needs at least one image file of assembly code to run a program. To run use `lc-3-vm <path-to-image-file-1> .. <path-to-image-file-n>`, with image files being a `.obj` file. 

For example:
```bash
make build
lc-3-vm examples/rogue.obj
```

- You can also run the examples by using `make example-2048` and `make example-rogue`.
- Additionally you can run `make all` to run the program and also run the tests, run clippy and format the code.
- Use `make test` to run the tests, use `make fmt` to format the code and `make clippy` to run clippy.
