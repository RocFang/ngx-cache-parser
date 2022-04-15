# ngx-cache-parser

A simple tool to parse the nginx proxy_cache file's binary header.

The simplest way is to read the data from cache file and cast it to `ngx_http_file_cache_header_t` directly in c.

This project is just a showcase for `std::slice::from_raw_parts_mut`, etc.

## Usage

```console
ngx-cache-parser <path-of-cache-file>
```
