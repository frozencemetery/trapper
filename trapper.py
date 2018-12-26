#!/usr/bin/env python3

import curses
import os
import pty
import select
import signal
import sys
import termios
import tty

import pyte

def update_screen(stdscr, screen):
    for row in screen.dirty:
        line = screen.buffer[row]
        stdscr.move(row, 0)
        stdscr.clrtoeol()
        data = "".join([line[x].data for x in range(screen.columns)])
        stdscr.addstr(data)
        pass
    screen.dirty.clear()
    stdscr.move(screen.cursor.y, screen.cursor.x)
    stdscr.refresh() # if not using the getch loop, you need to this
    return

def curses_main(stdscr):
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

    update_screen(stdscr, screen)

    inqueue = []
    while True:
        wl = [] if len(inqueue) == 0 else [amaster]
        rl, wl, xl = select.select([amaster, stdin], wl, [amaster, stdin])
        if xl != []:
            break
        if amaster in rl:
            data = amaster.read(1024)
            stream.feed(data)
            update_screen(stdscr, screen)
            pass
        if stdin in rl:
            elt = sys.stdin.read(1)
            inqueue.append(elt)
            pass
        if len(inqueue) != 0 and amaster.writable:
            elt = inqueue.pop(0)
            amaster.write(elt.encode())
            pass
        continue

    os.kill(pid, signal.SIGTERM)
    pass

if __name__ == "__main__":
    exit(curses.wrapper(curses_main))
