# VM that uses LC-3 architecture

This project is an implementation of a LC-3 virtual machine, following [this tutorial.](https://www.jmeiners.com/lc3-vm/).

### How to run
- Clone the repository with `git clone`, and enter de directory containing the VM code.
- The VM needs at least one image file of assebly code to run a program. To run use `make run FILE=<path-to-image-file>`, with image files being a `.obj` file. 
- By default (`make run`) it will run the `2048.obj` file.

For example:
```bash
make run FILE=examples/rogue.obj
```

- You can also run the examples by using `make example-2048` and `make example-rogue`.
- Additionally you can run `make all` to run the program and also run the tests, run clippy and format the code.
- Use `make test` to run the tests, use `make fmt` to format the code and `make clippy` to run clippy.
