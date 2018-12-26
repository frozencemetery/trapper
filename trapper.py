#!/usr/bin/env python3

import os
import pty
import select
import signal
import sys
import termios
import tty

import pyte

def update_screen(screen):
    print(*screen.display, sep="\n")
    return

if __name__ == "__main__":
    screen = pyte.Screen(78, 24)
    stream = pyte.ByteStream(screen)

    pid, amaster = pty.fork()
    if pid == 0: # chile
        os.execvpe("/usr/games/nethack", ["/usr/games/nethack"],
                   env=dict(TERM="linux", COLUMNS="78", LINES="24",
                            NETHACKOPTIONS=f"{os.getenv('HOME')}/.nethackrc"))
        exit(-1)

    # python3 broke setting stdin unbuffered.  Thanks, python3.
    stdin = os.fdopen(sys.stdin.fileno(), "rb", buffering=0)
    tty.setcbreak(stdin, termios.TCSANOW)

    # this wasn't python3, but I'm still mad about it because of how long it
    # took to figure out
    amaster = os.fdopen(amaster, "w+b", buffering=0)

    update_screen(screen)

    inqueue = []
    while True:
        wl = [] if len(inqueue) == 0 else [amaster]
        rl, wl, xl = select.select([amaster, stdin], wl, [amaster, stdin])
        if xl != []:
            break
        if amaster in rl:
            data = amaster.read(1024)
            stream.feed(data)
            update_screen(screen)
            pass
        if amaster in wl:
            elt = inqueue.pop(0)
            amaster.write(elt.encode())
            pass
        if stdin in rl:
            elt = sys.stdin.read(1)
            inqueue.append(elt)
            pass
        continue

    update_screen(screen)
    os.kill(pid, signal.SIGTERM)
    pass
