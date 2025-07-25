#! /bin/sh

# See https://doc.rust-lang.org/rustc/instrument-coverage.html

laze build -c -s nightly --builders host  --global -m --keep-going=0 -s coverage coverage 


# add doctests to the objects

for file in  target/debug/doctestbins/*/rust_out ;
do 
  if [ -f "$file" ]; then
    printf "%s %s " -object $file >> build/profile/objects;
  fi
done

RUSTUP_TOOLCHAIN=$(scripts/rust-toolchain.sh)
sysroot=$(rustc --print sysroot)

profdata=$(find $sysroot -name "llvm-profdata")
cov=$(find $sysroot -name "llvm-cov")


objects=$(cat build/profile/objects)

${profdata} merge -sparse build/profile/*.profraw -o build/profile/output.profdata  

${cov} export ${objects} -Xdemangler=rustfilt -instr-profile=build/profile/output.profdata -format=lcov --ignore-filename-regex='/.cargo' --ignore-filename-regex='rustc/' --ignore-filename-regex='/.rustup' > build/profile/coverage.txt