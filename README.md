# Web benchmarking tool

## Description

This tool is used to benchmark how many requests per second a web server can handle.

## Usage

```sh
$ git clone https://github.com/Quozul/web_bench.git
$ cd web_bench
$ cargo install
$ web_bench --help
```

## Example output

```sh
$ ./web_bench --duration 1 --threads 1 2 4 6 8 10 12 14 16 --host http://localhost
1143 requests made in 1.000398941s on 1 thread(s). Requests per second: 1142.54
2008 requests made in 1.000452232s on 2 thread(s). Requests per second: 2007.09
2806 requests made in 1.002224107s on 4 thread(s). Requests per second: 2799.77
3065 requests made in 1.002163817s on 6 thread(s). Requests per second: 3058.38
2998 requests made in 1.023529795s on 8 thread(s). Requests per second: 2929.08
3218 requests made in 1.029873121s on 10 thread(s). Requests per second: 3124.66
2503 requests made in 1.032816618s on 12 thread(s). Requests per second: 2423.47
3048 requests made in 1.033789887s on 14 thread(s). Requests per second: 2948.37
2690 requests made in 1.037878178s on 16 thread(s). Requests per second: 2591.83
```
