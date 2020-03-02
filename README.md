# TMS2XYZ

## About

TMS2XYZ is a command line program for converting TMS style tile package directory structures to "XYZ" format by renaming all the image names.

## Installation

### Linux
Download the binary and run it or clone the repo and install with `cargo install`.

### Windows
Not currently supported.

## Usage

Pass the directory to the program: `tms2xyz /path/to/dir/`. 

Use the `--dryrun` flag to check outputs before converting file names:

`tms2xyz --dryrun /path/to/dir/`

Use the `--type` flag to change the image extension (defaults to `png`):

`tms2xyz --type jpg /path/to/dir/`

Run it with the `-v` flag to see file renaming output (this slows it down significantly).

`tms2xyz -v --type jpg /path/to/dir/`

## Tests

Tests need to be run sequentially so use `cargo test -- --test-threads=1` for running them.

