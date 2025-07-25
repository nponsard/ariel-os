#! /bin/sh

# See https://doc.rust-lang.org/rustc/instrument-coverage.html

laze build -c -s nightly --builders host  --global -m --keep-going=0 -s coverage coverage 


# add doctests to the objects

for file in  target/debug/doctestbins/*/rust_out ;
do 
  [[ -x $file ]] && printf "%s %s " -object $file >> build/profile/objects;
done 

objects=$(cat build/profile/objects)

llvm-profdata merge -sparse build/profile/*.profraw -o build/profile/output.profdata  

llvm-cov export ${objects} -Xdemangler=rustfilt -instr-profile=build/profile/output.profdata -format=lcov --ignore-filename-regex='/.cargo' --ignore-filename-regex='rustc/' --ignore-filename-regex='/.rustup' > build/profile/coverage.txt