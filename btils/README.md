# btils - the BELLE Utils

Currently, there is one program in the BTILS program set - bfmt - the BELLE formatter, written in Haskell and in OCaml and C++.

bfmt can be compiled with `c++ -o bfmt bfmt/bfmt.cpp`, `ghc -o bfmt bfmt/bfmt.hs` or `ocamlc -o bfmt bfmt/bfmt.ml`, and can take one or many CLI arguments on assembly code written for BELLE, formatting it.

Lines that are not subroutines are indented (4 spaces), whilst lines with subroutine definitions are not.
