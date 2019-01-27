# file-expert(1) -- Guess the file type

## SYNOPSIS

  `file-expert` <FILE>... <br>

## DESCRIPTION

A file type recognition system similar to GitHub/linguist in Prolog.

## BUILDING

Just run `make`.

## HISTORY

Started around November 2017. Replaced with a Nim implementation in December
2018, because at this point the GitHub/linguist heuristics were a huge mess of
`if ... else` Ruby spaghetti code.

This project was archived until January 2019. My Nim implementation was broken
by the compiler update. While fixing it I noticed that the GitHub/linguist
heuristics were refactored to a YAML file. This opened up the possibility to
easily autogenerate the heuristics rules. So I started working on the Prolog
implementation again.

## AUTHOR

Written by Bahtiar \`kalkin-\` Gadimov.
