# README

Comparison of reading large amounts of data in Ruby and Rust.
PS: Using Rust to seed the DB, because it took about 1.5 hours to seed a large amount of data in Ruby

- How To Use
  1. docker-compose up
  1. You can see the log of DB seed submission.
  1. After running rails server, access localhost:3000.
  1. You can compare large csv download in Ruby and Rust(ffi).
  1. Don't forget to execute [docker-compose down -v] command when you are done comparing.
