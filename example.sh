#!/bin/sh

WASM="./rs-stop-words-print.wasm"

print_supported_languages(){
    wasmtime run "${WASM}" --print-supported-languages
}

print_stop_words(){
    wasmtime run "${WASM}" --language=English
}

print_supported_languages | sort | cat -n | head -3

print_stop_words | sort | cat -n | tail -3
