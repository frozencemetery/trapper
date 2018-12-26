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

PAIRS = {
    "default": lambda: curses.color_pair(1),
    "white": lambda: curses.color_pair(0), # hardcoded
    "brown": lambda: curses.color_pair(2), # brown ~= yellow I guess
    "cyan": lambda: curses.color_pair(3),
    "green": lambda: curses.color_pair(4),
    "magenta": lambda: curses.color_pair(5),
    "black": lambda: curses.color_pair(6),
    "red": lambda: curses.color_pair(7),
    "blue": lambda: curses.color_pair(8),
}

def update_screen(stdscr, screen):
    for row in screen.dirty:
        line = screen.buffer[row]
        stdscr.move(row, 0)
        stdscr.clrtoeol()
        for x in range(screen.columns):
            c = line[x]
            boldp = curses.A_BOLD if c.bold else 0
            revp = curses.A_REVERSE if c.reverse else 0

            # addch seems to mishandle attrs, so use addstr
            stdscr.addstr(c.data, PAIRS[c.fg]() | boldp | revp)
            pass
        pass
    screen.dirty.clear()
    stdscr.move(screen.cursor.y, screen.cursor.x)
    stdscr.refresh() # if not using the getch loop, you need to this
    return

def curses_main(stdscr):
    # ugh
    curses.use_default_colors()
    curses.init_pair(1, -1, -1)
    curses.init_pair(2, curses.COLOR_YELLOW, -1)
    curses.init_pair(3, curses.COLOR_CYAN, -1)
    curses.init_pair(4, curses.COLOR_GREEN, -1)
    curses.init_pair(5, curses.COLOR_MAGENTA, -1)
    curses.init_pair(6, curses.COLOR_BLACK, -1)
    curses.init_pair(7, curses.COLOR_RED, -1)
    curses.init_pair(8, curses.COLOR_BLUE, -1)

    screen = pyte.Screen(79, 24)
    stream = pyte.ByteStream(screen)

    pid, amaster = pty.fork()
    if pid == 0: # chile
        os.execvpe("/usr/games/nethack", ["/usr/games/nethack"],
                   env=dict(TERM="linux", COLUMNS="79", LINES="24",
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
