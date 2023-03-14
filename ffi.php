<?php 

function pomsky(string $pattern): string|null {
    static $ffi = null;

    if ($ffi === null) {
        $ffi = FFI::cdef(
            "char* pomsky_create(const char *pattern);",
            "pomsky_php_ext/target/debug/libphp_pomsky.so"
        );
    }
    
    $result = $ffi->pomsky_create($pattern);

    if ($result === null) {
        return null;
    }

    return FFI::string($result);
}

echo pomsky("ranhnge '0'-'255'");

