//! This largely follows <https://pcg-random.org/posts/how-to-test-with-testu01.html>.

#![allow(static_mut_refs)]

mod defer;

use std::{
    ffi::{c_char, c_int, c_long, c_uint, c_void},
    io::{BufReader, Read, StdinLock, stdin},
    mem::MaybeUninit,
    process, ptr,
    str::FromStr,
};

use larpa::{Command, types::Verbosity};

use crate::defer::defer;

/// Fucking "le bool" is what they called it. Are you serious. How french can you be.
#[allow(non_camel_case_types)]
type lebool = c_int;

#[allow(non_camel_case_types)]
type unif01_Gen = c_void;
#[allow(non_camel_case_types)]
type scomp_Res = c_void;

#[link(name = "testu01")]
unsafe extern "C" {
    static mut swrite_Basic: lebool;
    static mut swrite_Host: lebool;

    fn unif01_CreateExternGenBits(
        name: *mut c_char,
        f_Bits: extern "C" fn() -> c_uint,
    ) -> *mut unif01_Gen;
    fn unif01_DeleteExternGenBits(g: *mut unif01_Gen);

    fn bbattery_SmallCrush(g: *mut unif01_Gen);
    fn bbattery_Crush(g: *mut unif01_Gen);
    fn bbattery_BigCrush(g: *mut unif01_Gen);

    fn scomp_CreateRes() -> *mut scomp_Res;
    fn scomp_DeleteRes(res: *mut scomp_Res);
    fn scomp_LinearComp(
        g: *mut unif01_Gen,
        res: *mut scomp_Res,
        N: c_long,
        n: c_long,
        r: c_int,
        s: c_int,
    );
}

/// TestU01 driver.
///
/// Pipe random 32-bit integers into stdin to run statistical tests on them.
#[derive(Command)]
struct Cli {
    /// Reverse the bits of each 32-bit word before feeding them into the test.
    #[larpa(flag, name = ["-r", "--reverse-bits"])]
    reverse_bits: bool,

    #[larpa(flag, name = ["-v", "--verbose"])]
    verbose: Verbosity,

    #[larpa(flag, name = ["-q", "--quiet"], inverse_of = "verbose")]
    _quiet: (),

    /// Comma-separated list of test batteries to run; valid names are 'SmallCrush', 'Crush', 'BigCrush' and 'LinComp'.
    #[larpa(name = ["-T", "--tests"], default)]
    tests: TestList,
}

struct TestList(Vec<TestBattery>);

impl Default for TestList {
    fn default() -> Self {
        Self(vec![TestBattery::SmallCrush])
    }
}

impl FromStr for TestList {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.split(',')
                .map(TestBattery::from_str)
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

#[derive(Debug)]
enum TestBattery {
    SmallCrush,
    Crush,
    BigCrush,
    LinComp,
}

impl FromStr for TestBattery {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &*s.to_ascii_lowercase() {
            "smallcrush" => Ok(Self::SmallCrush),
            "crush" => Ok(Self::Crush),
            "bigcrush" => Ok(Self::BigCrush),
            "lincomp" => Ok(Self::LinComp),
            _ => Err(format!("unknown test battery '{s}'")),
        }
    }
}

static mut STDIN: MaybeUninit<BufReader<StdinLock<'static>>> = MaybeUninit::zeroed();

extern "C" fn generator<const REVERSE_BITS: bool>() -> c_uint {
    unsafe {
        let mut bytes = [0; 4];
        match STDIN.assume_init_mut().read_exact(&mut bytes) {
            Ok(()) => {
                let mut int = c_uint::from_ne_bytes(bytes);
                if REVERSE_BITS {
                    int = int.reverse_bits();
                }
                int
            }
            Err(e) => {
                eprintln!("error while reading from stdin: {e}");
                process::exit(1);
            }
        }
    }
}

fn main() {
    unsafe {
        STDIN.as_mut_ptr().write(BufReader::new(stdin().lock()));
    }

    let cli = Cli::from_args();

    if cli.verbose.get() < 0 {
        unsafe {
            swrite_Basic = 0;
        }
    }
    if cli.verbose.get() == 0 {
        unsafe {
            swrite_Host = 0;
        }
    }

    let func = if cli.reverse_bits {
        generator::<true>
    } else {
        generator::<false>
    };
    let g = unsafe { unif01_CreateExternGenBits(ptr::null_mut(), func) };
    let _d = defer(|| unsafe { unif01_DeleteExternGenBits(g) });

    let mut batteries = cli.tests.0;
    if batteries.is_empty() {
        batteries.push(TestBattery::SmallCrush);
    }

    eprintln!("Running the following test batteries: {batteries:?}");

    for bat in batteries {
        match bat {
            TestBattery::SmallCrush => unsafe { bbattery_SmallCrush(g) },
            TestBattery::Crush => unsafe { bbattery_Crush(g) },
            TestBattery::BigCrush => unsafe { bbattery_BigCrush(g) },
            TestBattery::LinComp => unsafe {
                // FIXME: make these configurable
                const SIZES: &[c_long] = &[250, 500, 1000, 5000, 25000, 50000, 75000];

                let res = scomp_CreateRes();
                for &size in SIZES {
                    scomp_LinearComp(g, res, 1, size, 0, 1);
                }
                scomp_DeleteRes(res);
            },
        }
    }
}
