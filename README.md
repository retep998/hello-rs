I hereby challenge everyone to write Rust programs on their platform of choice that print out "Hello world!" with a newline to the console with a binary that is as small as possible.

Rules:
* All Rust libraries must be statically linked (including libstd and friends).
* Aside from Rust libraries, you can only depend on libc and system libraries.
* No assembly, not even inline assembly.
* All code must be Rust.

Windows
-------
In the `windows` directory.

With `x86_64-pc-windows-msvc` the following command creates a 1,536 byte .exe:
`cargo rustc --release -- -Clink-args="/ENTRY:entry_point /ALIGN:16"`

This entry has the advantage of using nothing more than stable Rust and still having access to all of libstd.

Something else
--------------
In the `linux` directory.

I dunno.
