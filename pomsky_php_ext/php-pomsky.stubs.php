<?php

// Stubs for php-pomsky

namespace Chrono {
    class LocalDateTime {
        public static function now(): \Chrono\LocalDateTime {}

        public function format(string $format): string {}
    }
}

namespace pomsky {
    function create(string $pattern): string {}
}
