#!/usr/bin/env python3

import os
import pty
import select
import signal
import sys
import termios
import tty

import pyte

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

    inqueue = []
    while True:
        wl = [] if len(inqueue) == 0 else [amaster]
        rl, wl, xl = select.select([amaster, stdin], wl, [amaster, stdin])
        if xl != []:
            break
        elif amaster in rl:
            data = os.read(amaster, 1024)
            stream.feed(data)
            print(*screen.display, sep="\n")
            continue
        elif amaster in wl:
            elt = inqueue.pop(0)
            os.write(amaster, elt)
            continue
        elif stdin in rl:
            elt = stdin.read(1)
            inqueue.append(elt)
            continue

        break

    print(*screen.display, sep="\n")
    os.kill(pid, signal.SIGTERM)
    exit(0)
