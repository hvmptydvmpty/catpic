.. -*- fill-column: 128 -*-

.. _catpic

===============
Picture Catalog
===============

Rust study: local picture file catalog with emphasis on identifying
duplicates and adding missing `Exif
<https://en.wikipedia.org/wiki/Exif>`_ fields e.g. dates.

Offline
=======

When in airplane mode, Rust setup out of the box is surprisingly
functional, here are a few easy hints:

1. Start a local server for Rust documentation, available on
   `<http://[::1]:8001>`_ or `<http://localhost:8001>`_;
   alternatively, can use `rustup doc` without a Web server::
     
     python3 -m http.server --bind localhost --directory "$(dirname $(rustup doc --path))" 8001 &

2. Generate documentation for the project and its dependencies and
   place in ``target/doc/``; use a similar Python HTTP server as above
   to view it nicely formatted and browsable on
   `<http://[::1]:8002/>`_ or `<http://localhost:8002>`_. I didn't
   find ``--open`` option all that useful due to poor browser
   rendering of files served from local file system::
     
     cargo doc --document-private-items

     python3 -m http.server --bind localhost --directory ./target/doc 8002 &

3. One more notable feature in the previous item is that source code
   of dependency crates is formatted in ``target/doc/src`` and is
   convenient for studying their methods.

References
==========

- `Cargo <https://doc.rust-lang.org/cargo/reference/manifest.html>`_
- `Cargo lock
  <https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html>`_
