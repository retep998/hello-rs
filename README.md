I hereby challenge everyone to write Rust programs on their platform of choice that print out "Hello world!" with a newline to the console with a binary that is as small as possible.

Rules:
* All Rust libraries must be statically linked (including libstd and friends).
* Aside from Rust libraries, you can only depend on libc and system libraries.
* No assembly, not even inline assembly.
* All code must be Rust.

Windows
-------
In the `windows` directory.

With `x86_64-pc-windows-msvc` the following command creates a 1,312 byte .exe:
`cargo rustc --release -- -Clink-args="/ENTRY:entry_point /ALIGN:16 /SUBSYSTEM:CONSOLE /NODEFAULTLIB"`

This entry requires nightly Rust, but only depends on libcore and kernel32.dll.

Something else
--------------
In the `linux` directory.

I dunno.
