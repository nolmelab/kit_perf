# kit_perf

performance benchmark of various backend components including sql databases, redis, tcp, 
and file io written in rust.

There are times to check diverse infra components of backend servers, but I have not 
found an easy to use and integrated one. 

kit-perf is a tool to check performance of: 

- file io 
  - test file io performance of a system

- databases 
  - mysql, sql server, postgresql 
  - redis 

- tcp 
  - communication test with diverse payload with various connections.

