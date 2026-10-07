<?php

echo pomsky\create("range '0'-'255'") . "\n";

$time = Chrono\LocalTime::now();

echo $time->format("%Y") . "\n";;
