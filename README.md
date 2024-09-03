# VM that uses LC-3 architecture

Following [this tutorial.](https://www.jmeiners.com/lc3-vm/)

### How to run
- Clone the repository with `git clone`.
- Currently the VM is run by using `cargo run <image-file-1> .. <image-file-k>`, with image files being a `.obj` file containing some assembly code.
- You can also run the examples by using `make example-2048` and `make example-rogue`.
- Additionally you can run `make all` to run the program and also run the tests, run clippy and format the code.
- Use `make test` to run the tests, use `make fmt` to format the code and `make clippy` to run clippy.
