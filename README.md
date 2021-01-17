# openal-soft-sys

supported openal-soft rev: **855a8c0cd9c79d3e708d037811b04a821f95a5bc**.

# requires

## bindgen **command line tool**

```shell script
cargo install bindgen
```

## openal-soft

```shell script
cd <local_repository_root>
git clone https://github.com/kcat/openal-soft.git
```

# build

## for Windows

### cmake and install openal-soft

```shell script
cd <OPENAL_SOFT_ROOT>
mkdir build
cd build
cmake -G "Visual Studio 15 2017 Win64" -DCMAKE_INSTALL_PREFIX=../install ..
@rem use linker
cmake --build . --config Debug --target INSTALL
```

# set environment variables

```shell script
set LIBCLANG_PATH=<LLVM_HOME>\bin
set OPENAL_SOFT_PATH=<OPENAL_SOFT_HOME>
```

# generate binding

```shell script
cd <OPENAL_SOFT_SYS_ROOT>\genbinding\win
gen al & gen alc & gen alext
```

## note

bat file args.

```shell script
gen <binding-prefix>
```

binding-prefix.

- al
- alc
- alext

# build openal-soft-sys

```shell script
cd <OPENAL_SOFT_SYS_ROOT>
@rem OpenAL32.lib link here!
cargo build --example openal-info
```
