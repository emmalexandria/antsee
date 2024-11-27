#!/usr/bin/env fish

argparse 'cargo browser' -- $argv or return
if set $_flag_cargo
    cargo watch -s 'cargo doc'
end
if $_flag_s = 2
    browser-sync ./target/doc -w &
end
