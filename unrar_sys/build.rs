extern crate cc;

use std::env::set_var;

fn main() {
    if cfg!(windows) {
        println!("cargo:rustc-flags=-lpowrprof");
        set_var("LDFLAGS", "-pthread");
    } else {
        set_var("LDFLAGS", "-pthread");
    }

    set_var("LIBFLAGS", "-fPIC");
    if cfg!(target_env = "msvc") {
        set_var("CXXFLAGS", "-O2");
        println!("cargo:rustc-link-lib=shell32");
    } else {
        set_var("CXXFLAGS", "-O2 -Wno-macro-redefined -Wno-dangling-else -Wno-logical-op-parentheses");
    }
    cc::Build::new()
        .cpp(true) // Switch to C++ library compilation.
        .define("_FILE_OFFSET_BITS", Some("64"))
        .define("_LARGEFILE_SOURCE", None)
        .define("RAR_SMP", None)
        .define("RARDLL", None)
        .file("vendor/unrar/rar.cpp")
        .file("vendor/unrar/strlist.cpp")
        .file("vendor/unrar/strfn.cpp")
        .file("vendor/unrar/pathfn.cpp")
        .file("vendor/unrar/smallfn.cpp")
        .file("vendor/unrar/global.cpp")
        .file("vendor/unrar/file.cpp")
        .file("vendor/unrar/filefn.cpp")
        .file("vendor/unrar/filcreat.cpp")
        .file("vendor/unrar/archive.cpp")
        .file("vendor/unrar/arcread.cpp")
        .file("vendor/unrar/unicode.cpp")
        .file("vendor/unrar/system.cpp")
        .file("vendor/unrar/isnt.cpp")
        .file("vendor/unrar/crypt.cpp")
        .file("vendor/unrar/crc.cpp")
        .file("vendor/unrar/rawread.cpp")
        .file("vendor/unrar/encname.cpp")
        .file("vendor/unrar/resource.cpp")
        .file("vendor/unrar/match.cpp")
        .file("vendor/unrar/timefn.cpp")
        .file("vendor/unrar/rdwrfn.cpp")
        .file("vendor/unrar/consio.cpp")
        .file("vendor/unrar/options.cpp")
        .file("vendor/unrar/errhnd.cpp")
        .file("vendor/unrar/rarvm.cpp")
        .file("vendor/unrar/secpassword.cpp")
        .file("vendor/unrar/rijndael.cpp")
        .file("vendor/unrar/getbits.cpp")
        .file("vendor/unrar/sha1.cpp")
        .file("vendor/unrar/sha256.cpp")
        .file("vendor/unrar/blake2s.cpp")
        .file("vendor/unrar/hash.cpp")
        .file("vendor/unrar/extinfo.cpp")
        .file("vendor/unrar/extract.cpp")
        .file("vendor/unrar/volume.cpp")
        .file("vendor/unrar/list.cpp")
        .file("vendor/unrar/find.cpp")
        .file("vendor/unrar/unpack.cpp")
        .file("vendor/unrar/headers.cpp")
        .file("vendor/unrar/threadpool.cpp")
        .file("vendor/unrar/rs16.cpp")
        .file("vendor/unrar/cmddata.cpp")
        .file("vendor/unrar/ui.cpp")
        .file("vendor/unrar/filestr.cpp")
        .file("vendor/unrar/scantree.cpp")
        .file("vendor/unrar/dll.cpp")
        .file("vendor/unrar/qopen.cpp")
        .compile("libunrar.a");
}
