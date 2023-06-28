<div align="center" id="top">
  <img src="./.github/dnsxy.jpg" alt="Dnsxy" width=50% />

  &#xa0;

  <!-- <a href="https://dnsxy.netlify.app">Demo</a> -->
</div>

<h1 align="center">DNSxy</h1>

<p align="center">
  <img alt="Github top language" src="https://img.shields.io/github/languages/top/y0k4i-1337/dnsxy?color=56BEB8">

  <img alt="Github language count" src="https://img.shields.io/github/languages/count/y0k4i-1337/dnsxy?color=56BEB8">

  <img alt="Repository size" src="https://img.shields.io/github/repo-size/y0k4i-1337/dnsxy?color=56BEB8">

  <img alt="License" src="https://img.shields.io/github/license/y0k4i-1337/dnsxy?color=56BEB8">

  <!-- <img alt="Github issues" src="https://img.shields.io/github/issues/y0k4i-1337/dnsxy?color=56BEB8" /> -->

  <!-- <img alt="Github forks" src="https://img.shields.io/github/forks/y0k4i-1337/dnsxy?color=56BEB8" /> -->

  <!-- <img alt="Github stars" src="https://img.shields.io/github/stars/y0k4i-1337/dnsxy?color=56BEB8" /> -->
</p>

<!-- Status -->

<!-- <h4 align="center">
	🚧  Dnsxy 🚀 Under construction...  🚧
</h4>

<hr> -->

<p align="center">
  <a href="#dart-about">About</a> &#xa0; | &#xa0;
  <a href="#sparkles-features">Features</a> &#xa0; | &#xa0;
  <a href="#rocket-technologies">Technologies</a> &#xa0; | &#xa0;
  <a href="#white_check_mark-requirements">Requirements</a> &#xa0; | &#xa0;
  <a href="#checkered_flag-starting">Starting</a> &#xa0; | &#xa0;
  <a href="#memo-license">License</a> &#xa0; | &#xa0;
  <a href="https://github.com/y0k4i-1337" target="_blank">Author</a>
</p>

<br>

## :dart: About ##

`dnsxy` is a DNS server written in Rust that allows you to run a local DNS server and forward requests to other servers using a SOCKS proxy. It provides a convenient way to intercept and manipulate DNS queries and responses.

## :sparkles: Features ##

:heavy_check_mark: Forward DNS queries through a SOCKS proxy to remote nameservers;\
:heavy_check_mark: Support for specifying multiple nameservers using a comma-separated list or a file;\
:heavy_check_mark: Option to use TCP instead of UDP for DNS queries;\
:heavy_check_mark: Easy setup and configuration through command-line options;\
:heavy_check_mark: Built-in DNS server implementation using the trust-dns library.


## :rocket: Technologies ##

The following tools were used in this project:

- [Rust](https:/rust-lang.org)
- [Trust-dns](https://trust-dns.org/)
- [socks](https://docs.rs/socks/latest/socks/)


## :white_check_mark: Requirements ##

Before starting :checkered_flag:, you need to have [Git](https://git-scm.com) and [Rust](https://rust-lang.org) installed.

## :checkered_flag: Starting ##

```bash
# Clone this project
$ git clone https://github.com/y0k4i-1337/dnsxy

# Access
$ cd dnsxy

# Build the project
$ cargo build --release

# Run the DNS server
$ cargo run --release -- [options]

# The server will initialize in <localhost:53> by default
```

## :memo: License ##

This project is under license from Apache-2.0. For more details, see the [LICENSE](LICENSE) file.


Made with :heart: by <a href="https://github.com/y0k4i-1337" target="_blank">y0k4i</a>

&#xa0;

<a href="#top">Back to top</a>
