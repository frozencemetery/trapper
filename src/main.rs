use std::ptr;
mod tsm;

extern crate libc;
use std::char::from_u32;

/* I'm so unhappy about this */
fn shit_strerror() {
    unsafe {
        let e = *libc::__errno_location();
        let s = libc::strerror(e);
        let len = libc::strlen(s);
        let slice = std::slice::from_raw_parts(s as *mut u8, len);
        let s = std::str::from_utf8(slice).unwrap();
        println!("{}: {}", e, s);
    }
}

fn main() {
    let mut screen = ptr::null_mut();
    let err = unsafe {
        tsm::tsm_screen_new(&mut screen, None, ptr::null_mut())
    };
    assert_eq!(0, err);

    let err = unsafe { tsm::tsm_screen_resize(screen, 80, 24) };
    assert_eq!(0, err);

    let res = unsafe { tsm::tsm_screen_get_width(screen) };
    assert_eq!(80, res);

    let res = unsafe { tsm::tsm_screen_get_height(screen) };
    assert_eq!(24, res);

    let attr = tsm::tsm_screen_attr {
        fccode: 0,
        bccode: 0,
        fr: 0,
        fg: 0,
        fb: 0,
        br: 0,
        bg: 0,
        bb: 0,
        _bitfield: 0,
    };
    for c in "hello world".chars() {
        unsafe { tsm::tsm_screen_write(screen, c as tsm::tsm_symbol_t, &attr); }
    }

    struct Output { string: String }
    extern "C" fn draw_cb(_: *mut tsm::tsm_screen,
                          _: u32,
                          ch: *const u32,
                          _: usize,
                          _: libc::c_uint,
                          _: libc::c_uint,
                          _: libc::c_uint,
                          _: *const tsm::tsm_screen_attr,
                          _: tsm::tsm_age_t,
                          output: *mut libc::c_void) -> libc::c_int {
        let output: &mut Output = unsafe { &mut *(output as *mut Output) };
        let c;
        unsafe {
            if *ch == 0 {
                return 0;
            } else {
                c = from_u32(*ch).unwrap()
            }
        };
        output.string.push(c);
        0
    }


    extern "C" fn write_cb(_: *mut tsm::tsm_vte,
                           _: *const libc::c_char,
                           _: isize,
                           _: *mut libc::c_void) {};
    let mut vte = ptr::null_mut();
    let r = unsafe { tsm::tsm_vte_new(&mut vte, screen, Some(write_cb),
                                      ptr::null_mut(), None, ptr::null_mut()) };
    assert_eq!(0, r);

    let fd = unsafe {
        libc::posix_openpt(libc::O_RDWR | libc::O_NOCTTY |
                           libc::O_CLOEXEC | libc::O_NONBLOCK)
    };
    assert!(fd > 0);

    let mut files: Vec<libc::c_int> = vec![0, 0, 0];
    let r = unsafe { libc::pipe2(files.as_mut_ptr(), libc::O_CLOEXEC) };
    assert!(r == 0);

    let r = unsafe { libc::fork() };
    assert!(r >= 0);
    if r == 0 {
        /* child */
        unsafe { libc::close(files[0]) };

        /* TODO(frozen) might need this? */
        /* let mut sigset; */
        /* unsafe { */
        /*     /\* hello there FFI *\/ */
        /*     sigset = std::mem::uninitialized(); */
        /*     libc::sigemptyset(&mut sigset); */
        /* }; */

        /* let r = unsafe { */
        /*     libc::pthread_sigmask(libc::SIG_SETMASK, &mut sigset, */
        /*                           ptr::null_mut()) */
        /* }; */
        /* assert!(r == 0); */

        /* for i in 1..libc::SIGUNUSED { */
        /*     let r = unsafe { libc::signal(i, libc::SIG_DFL) }; */
        /*     if r == libc::SIG_ERR { */
        /*         println!("Signal error: {}", i); */
        /*     } */
        /* } */

        let r = unsafe { libc::grantpt(fd) };
        assert!(r == 0);

        let r = unsafe { libc::unlockpt(fd) };
        assert!(r == 0);

        let slave_name = unsafe { libc::ptsname(fd) };
        assert!(slave_name != 0 as *mut i8);

        let slave = unsafe {
            libc::open(slave_name,
                       libc::O_RDWR | libc::O_CLOEXEC | libc::O_NOCTTY)
        };
        assert!(slave >= 0);

        let pid = unsafe { libc::setsid() };
        assert!(pid >= 0);

        let r = unsafe { libc::ioctl(slave, libc::TIOCSCTTY, 0) };
        assert!(r >= 0);

        unsafe { libc::close(fd) };

        let mut attr;
        let r = unsafe {
            attr = std::mem::uninitialized();
            libc::tcgetattr(slave, &mut attr)
        };
        assert!(r >= 0);

        /* TODO(frozen) the heck is this */
        attr.c_cc[libc::VERASE] = 0o10;

        let r = unsafe { libc::tcsetattr(slave, libc::TCSANOW, &mut attr) };
        assert!(r >= 0);

        let r = unsafe {
            let mut ws: libc::winsize = std::mem::zeroed();
            ws.ws_col = 80;
            ws.ws_row = 24;
            libc::ioctl(slave, libc::TIOCSWINSZ, &mut ws)
        };
        assert!(r >= 0);

        let r = unsafe { libc::dup2(slave, libc::STDIN_FILENO) };
        assert!(r != -1);
        let r = unsafe { libc::dup2(slave, libc::STDOUT_FILENO) };
        assert!(r != -1);
        let r = unsafe { libc::dup2(slave, libc::STDERR_FILENO) };
        assert!(r != -1);

        if slave > 2 {
            unsafe { libc::close(slave) };
        }

        /* sup, parent */
        let d = "k".as_ptr();
        /* TODO(frozen) handle failures better */
        let mut r = 0;
        while r == 0 {
            r = unsafe { libc::write(files[1], d as *const libc::c_void, 1) };
        }
        assert!(r == 1);

        unsafe { libc::close(files[1]) };

        /* IT'S TIME TO PLAY THE GAME */
        let env = ["TERM=xterm-256color\0".as_ptr() as *const i8,
                   ptr::null_mut()];
        let argv = ["/usr/bin/nethack\0".as_ptr() as *const i8,
                    ptr::null_mut()];
        let _ = unsafe { libc::execve(argv[0], &argv[0], &env[0]) };
        assert!(false);
    } else {
        /* parent */
        unsafe { libc::close(files[1]) };

        let mut r = 0;
        let mut d: u8 = 0;
        while r == 0 {
            unsafe {
                r = libc::read(files[0], std::mem::transmute(&mut d), 1);
            }
            println!("{:?}", d);
        }
        assert!(r == 1);
        assert!(d == 'k' as u8);

        let mut output = Output { string: "".to_string() };
        let output_ptr = &mut output as *mut _ as *mut libc::c_void;
        unsafe { tsm::tsm_screen_draw(screen, Some(draw_cb), output_ptr) };
        println!("{:?}", &output.string);
    }

    /* stdin.write(b"y  #quit\nyq"); */

    /* loop { */
    /*     let mut res = vec![]; */
    /*     stdout.read(&mut res); */
    /*     println!("{}", String::from_utf8(res).unwrap()); */
    /* } */
}
