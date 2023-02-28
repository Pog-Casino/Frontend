in WASM there are Rust-based workers

Some glue code is present in this directory, simply run `tsc`. For development,
run

```
inotifywait -m -e modify /var/log | 
   while read file_path file_event file_name; do 
       tsc
   done
```
