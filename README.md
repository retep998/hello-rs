I hereby challenge everyone to write Rust programs on their platform of choice that print out "Hello world!" with a newline to the console with a binary that is as small as possible.

Rules:
* All Rust libraries must be statically linked (including libstd and friends).
* Aside from Rust libraries, you can only depend on libc and system libraries.
* No assembly, not even inline assembly.
* All code must be Rust.

This is my entry:

With `x86_64-pc-windows-msvc` the following command creates a 1,408 byte .exe:
`cargo rustc --release -- -Clink-args="/ENTRY:entry_point /ALIGN:16 /MERGE:.text=.data /MERGE:.rdata=.data /MERGE:.pdata=.data"`
