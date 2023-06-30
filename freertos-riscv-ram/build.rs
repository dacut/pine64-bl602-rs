use {
    bindgen::{Builder as BindgenBuilder, CargoCallbacks},
    cc::Build,
    std::{
        env,
        error::Error,
        fs::File,
        ffi::OsString,
        io::{BufRead, BufReader},
        path::{Path, PathBuf},
        process::ExitCode,
    },
};

lazy_static::lazy_static! {
    static ref OUT_DIR: PathBuf = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR env var not set"));
    static ref MANIFEST_DIR_STR: OsString = env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR env var not set");
    static ref MANIFEST_DIR: PathBuf = PathBuf::from(&*MANIFEST_DIR_STR);
    static ref BL_IOT_SDK_DIR: PathBuf = MANIFEST_DIR.join("src").join("bl_iot_sdk");
    static ref COMPONENT_DIR: PathBuf = BL_IOT_SDK_DIR.join(COMPONENT_BASE_DIR);
}

const COMPONENT: &str = "freertos_riscv_ram";

const COMPONENT_BASE_DIR: &str = "components/bl602/freertos_riscv_ram";

const COMPONENT_INCLUDE_DIRS: &[&str] = &[
    "config",
    "panic",
    "portable/GCC/RISC-V",
    "portable/GCC/RISC-V/chip_specific_extensions/RV32F_float_abi_single",
];

const COMPONENT_SRCS: &[&str] = &[
    "event_groups.c",
    "list.c",
    "queue.c",
    "stream_buffer.c",
    "tasks.c",
    "timers.c",
    "misaligned/misaligned_ldst.c",
    "misaligned/fp_asm.S",
    "panic/panic_c.c",
    "portable/GCC/RISC-V/port.c",
    "portable/GCC/RISC-V/portASM.S",
    "portable/MemMang/heap_5.c",
];

const ADDITIONAL_INCLUDE_DIRS: &[&str] = &[];

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Failed to generate bindings: {}", e);
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Box<dyn Error + 'static>> {
    generate_rust_bindings()?;
    compile_library()
}

fn generate_rust_bindings() -> Result<(), Box<dyn Error + 'static>> {
    println!("cargo:rerun-if-changed=bindings.h");
    let builder = BindgenBuilder::default()
        .use_core()
        .parse_callbacks(Box::new(CargoCallbacks));
    let mut builder = FlaggableBindgenBuilder::from(builder);

    add_compiler_directives(&mut builder);
    add_defines(&mut builder);

    // Parse the version.txt file and set definitions.
    parse_version_txt(&mut builder, &BL_IOT_SDK_DIR)?;

    // Add -I directives
    add_include_dirs(&mut builder)?;

    let builder = builder.into_inner();
    let bindings = builder.header("bindings.h").generate()?;

    let out_path = OUT_DIR.join("bindings.rs");
    bindings.write_to_file(out_path)?;

    Ok(())
}

fn compile_library() -> Result<(), Box<dyn Error + 'static>> {
    let component_dir_str = COMPONENT_DIR.to_str().ok_or("component_dir is not UTF-8")?;
    let mut build = Build::new();

    add_compiler_directives(&mut build);
    add_defines(&mut build);
    parse_version_txt(&mut build, &BL_IOT_SDK_DIR)?;
    add_include_dirs(&mut build)?;

    for file in COMPONENT_SRCS {
        println!("cargo:rerun-if-changed={component_dir_str}/{file}");
        let file = COMPONENT_DIR.join(file);
        build.file(file);
    }

    build.compile(COMPONENT);

    Ok(())
}

fn parse_version_txt(
    builder: &mut impl Flaggable,
    bl_iot_sdk_dir: &Path,
) -> Result<(), Box<dyn Error + 'static>> {
    let version_txt = bl_iot_sdk_dir.join("version.txt");
    let fd = match File::open(&version_txt) {
        Ok(fd) => fd,
        Err(e) => {
            eprintln!("Failed to open {version_txt:?}: {e}");
            return Err(e.into());
        }
    };
    let reader = BufReader::new(fd);
    let mut lines = reader.lines();

    // First line is the SDK version.
    let Some(sdk_ver) = lines.next() else {
        return Err("version.txt missing SDK version".into());
    };
    let sdk_ver = sdk_ver?;
    builder.define("BL_SDK_VER", sdk_ver.trim());

    // Second line is the PHY version.
    let Some(sdk_phy_ver) = lines.next() else {
        return Err("version.txt missing PHY version".into());
    };
    let sdk_phy_ver = sdk_phy_ver?;
    builder.define("BL_SDK_PHY_VER", sdk_phy_ver.trim());

    // Third line is the RF version.
    let Some(sdk_rf_ver) = lines.next() else {
        return Err("version.txt missing RF version".into());
    };
    let sdk_rf_ver = sdk_rf_ver?;
    builder.define("BL_SDK_RF_VER", sdk_rf_ver.trim());

    Ok(())
}

fn add_compiler_directives(builder: &mut impl Flaggable) {
    builder.flag("-Os");
    builder.flag("-ffunction-sections");
    builder.flag("-fdata-sections");
    builder.flag("-fshort-enums");
    builder.flag("-ffreestanding");
    builder.flag("-fno-strict-aliasing");
    builder.flag("-march=rv32imfc");
    builder.flag("-mabi=ilp32f");
}

fn add_defines(builder: &mut impl Flaggable) {
    builder.define_empty("ARCH_RISCV");
    builder.define("BL_CHIP_NAME", r#""BL602""#);
    builder.define("__COMPONENT_NAME__", format!(r#""{COMPONENT}""#));
    builder.define("__COMPONENT_NAME_DEQUOTED__", COMPONENT);
    builder.define("portasmHANDLE_INTERRUPT", "interrupt_entry");
    builder.define(
        "configUSE_TICKLESS_IDLE",
        if cfg!(feature = "tickless") { "1" } else { "0" },
    );

    if cfg!(feature = "user-psram") {
        builder.define_empty("CONF_USER_ENABLE_PSRAM");
    }

    builder.define(
        "CONFIG_PSM_EASYFLASH_SIZE",
        if cfg!(easyflash_size_8k) {
            "8192"
        } else if cfg!(easyflash_size_16k) {
            "16384"
        } else {
            "4096"
        },
    );
}

fn add_include_dirs(builder: &mut impl Flaggable) -> Result<(), Box<dyn Error + 'static>> {
    // Add component includes first.
    for dir in COMPONENT_INCLUDE_DIRS {
        let dir = COMPONENT_DIR.join(dir);
        let dir = dir.to_str().ok_or("Component include dir is not UTF-8")?;
        builder.include(dir);
    }

    // Then additional includes.
    for dir in ADDITIONAL_INCLUDE_DIRS {
        let dir = BL_IOT_SDK_DIR.join(dir);
        let dir = dir.to_str().ok_or("Additional include dir is not UTF-8")?;
        builder.include(dir);
    }

    Ok(())
}

trait Flaggable {
    fn flag<S: AsRef<str>>(&mut self, flag: S);

    fn define_empty<S: AsRef<str>>(&mut self, var: S) {
        self.flag(format!("-D{}", var.as_ref()));
    }

    fn define<S1: AsRef<str>, S2: AsRef<str>>(&mut self, var: S1, value: S2) {
        self.flag(format!("-D{}={}", var.as_ref(), value.as_ref()));
    }

    fn include<S: AsRef<str>>(&mut self, dir: S) {
        eprintln!("Adding include directory: {}", dir.as_ref());
        self.flag(format!("-I{}", dir.as_ref()));
    }
}

struct FlaggableBindgenBuilder {
    builder: BindgenBuilder,
}

impl FlaggableBindgenBuilder {
    fn into_inner(self) -> BindgenBuilder {
        self.builder
    }
}

impl From<BindgenBuilder> for FlaggableBindgenBuilder {
    fn from(builder: BindgenBuilder) -> Self {
        Self { builder }
    }
}

impl Flaggable for FlaggableBindgenBuilder {
    fn flag<S: AsRef<str>>(&mut self, flag: S) {
        self.builder = self.builder.clone().clang_arg(flag.as_ref());
    }
}

impl Flaggable for Build {
    fn flag<S: AsRef<str>>(&mut self, flag: S) {
        self.flag(flag.as_ref());
    }

    fn define_empty<S: AsRef<str>>(&mut self, var: S) {
        self.define(var.as_ref(), None);
    }

    fn define<S1: AsRef<str>, S2: AsRef<str>>(&mut self, var: S1, value: S2) {
        self.define(var.as_ref(), Some(value.as_ref()));
    }

    fn include<S: AsRef<str>>(&mut self, dir: S) {
        self.include(dir.as_ref());
    }
}
