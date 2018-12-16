#!/usr/bin/env python3

import os
import pty
import select
import signal
import sys

import pyte

if __name__ == "__main__":
    screen = pyte.Screen(78, 24)
    stream = pyte.ByteStream(screen)

    pid, amaster = pty.fork()
    if pid == 0: # chile
        os.execvpe("/usr/games/nethack", ["/usr/games/nethack"],
                   env=dict(TERM="linux", COLUMNS="78", LINES="24"))
        exit(-1)

    while True:
        rl, wl, xl = select.select([amaster], [], [])
        if amaster in xl:
            break
        elif amaster not in rl: # u wot
            continue

        stream.feed(os.read(amaster, 1024))
        print(*screen.display, sep="\n")

    print(*screen.display, sep="\n")
    os.kill(pid, signal.SIGTERM)
    exit(0)
