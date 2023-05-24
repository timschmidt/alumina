# STM32F411CE (blackpill) Wiznet W5500 ethernet project written in Rust

This project is an example of running Wiznet W5500 ethernet controller on STM32F411CE also know as blackpill, using Rust. Implementation uses SPI, so data line needs to be connected to pin A7, this of course can be changed if you want to use SPI2 etc.
I'll also try to write down some general data, how to get it to run on microcontroller.

To get started with embedded rust, there's a really good book: [The Embedded Rust Book ](https://docs.rust-embedded.org/book/intro/index.html), I really recommend reading at least first few chapters as it describes how to get entire toolchain up and running. If you want, you can skip most of it, by using [probe-run](https://github.com/knurling-rs/probe-run) which enables flashing and running the code as if it was normal rust code.

Code also relies on [stm32f4xx-hal](https://github.com/stm32-rs/stm32f4xx-hal) which is, as name implies, a HAL written in rust for stm32f4 series.

To work with the Wiznet W5500 ethernet module I have used the [w5500-rs](https://github.com/newAM/w5500-rs) crate which provides a driver and network protocol support for the Wiznet W5500 chip in Rust.

## Important

To get this to work, you'll need to cross-compile the code and compile it with --release tag. To load it you can either use <b>GDB and OpenOCD</b>, or <b>probe-run</b>. I'll describe both approaches bellow. Most of this info is explained in more detail in above mentioned book and linked repos.

### Toolchain

This code needs to be cross-compiled to work on STM32 microcontroller, which is an Arm Cortex M4 with FPU core. More info about the chip itself can be found in [Reference manual PDF](https://www.st.com/resource/en/reference_manual/dm00119316-stm32f411xc-e-advanced-arm-based-32-bit-mcus-stmicroelectronics.pdf).


In order to cross-compile, you'll need to add target used for this microcontroller <b>thumbv7em-none-eabihf</b>, using following command
```
rustup target add thumbv7em-none-eabihf
```

As stated in The Embedded Rust Book, I also installed, cargo-binutils and cargo-generate.
More details can be found [here](https://docs.rust-embedded.org/book/intro/install.html).

At this point you can either use probe-run or GDB with OpenOCD to flash the code to microcontroller. 

Using probe-run is simpler and quicker, but you can always use GDB as well.

Following are the setup instructions for either option, you don't need both.

### Probe-run

Probe run enables you to run the code on microcontroller, as if it was normal rust code. More info about it can be found in [probe-run repo](https://github.com/knurling-rs/probe-run).

To install, run following command:
```
cargo install probe-run
```

If that doesn't work, refer to repo linked above. You might also need a ST-LINK driver, at least on Windows if you don't already have one. You can download it on [official ST website](https://www.st.com/en/development-tools/stsw-link009.html).

Once you have probe-run installed you can run code as described in one of the sections bellow, config file already contains setting for probe-run and stm32f411CEU blackpill.

### GDB & OpenOCD

Everything about installation and details about tools are explained in [The Embedded Rust Book, section Tooling and Installation
](https://docs.rust-embedded.org/book/intro/tooling.html)Following is a quick overview.

On the same page you can also find OS-Specific instructions for installing following needed tools:
- GDB for Arm programs
- OpenOCD to work with ST-LINK
- ST-LINK usb driver, if needed

Once all of these are installed, you can very installation is working with following command:
```
OpenOCD -f interface/stlink.cfg -f target/stm32f4x.cfg
```
Something along the following output should be produced:
```
...
Info : clock speed 2000 kHz
Info : STLINK V2J37S7 (API v2) VID:PID 0483:3748
Info : Target voltage: 3.240957
Info : stm32f4x.cpu: Cortex-M4 r0p1 processor detected
Info : stm32f4x.cpu: target has 6 breakpoints, 4 watchpoints
Info : starting gdb server for stm32f4x.cpu on 3333
Info : Listening on port 3333 for gdb connections
```
If it is not working, please refer to [following page](https://docs.rust-embedded.org/book/intro/install/verify.html).

## Project settings

For project to work correctly with HAL and given microcontroller, you'll also need <b>[.cargo/config](https://github.com/blaz-r/STM32F411-rust-neopixel/tree/main/.cargo/config)</b> and <b>[memory.x](https://github.com/blaz-r/STM32F411-rust-neopixel/tree/main/memory.x)</b> files. These are already present in this project and set up for blackpill.
- Config file is used to select the right target for building and in order for linker to work correctly. It also provides command for probe-run.
- Memory.x is used to specify where RAM and FLASH are mapped on microcontroller. This info is currently set for blackpill with 128kB ram and 512kB flash. Info about this can be found in [above specified reference manual for stm32f411](https://www.st.com/resource/en/reference_manual/dm00119316-stm32f411xc-e-advanced-arm-based-32-bit-mcus-stmicroelectronics.pdf)  in "Memory and bus architecture" section page 42.

Since the project uses HAL and smart-led crates as stated above, they are listed in [Cargo.toml](https://github.com/blaz-r/STM32F411-rust-neopixel/tree/main/Cargo.toml) under dependencies. There's also some other dependencies needed, more information about these can be found in above mentioned book and HAL repo.

## Building project

With everything properly configured, project can be simply built using following command
```
cargo build --release
```
-- release flag here was needed in my case, without this flag, the project didn't work correctly.


### Running project with probe-run

To run project with probe-run, use the following instruction from inside project folder:
```
cargo run --release
```
Probe-run, with help of config file, will take care of the rest. As with normal rust, using run will build the project, then flash and run it on microcontroller. For more info or in case of problems refer to [probe-run repo](https://github.com/knurling-rs/probe-run).


### Running project with GDB & OpenOCD

To load and run the project on microcontroller, you'll need OpenOCD and GDB for Arm, which you can install as described above.

OpenOCD serves as an intermediate between GDB and ST-LINK and GDB itself is used for debugging and loading the code to microcontroller.

Following are the command I use on my windows setup, it should be very similar for linux:
- first run OpenOCD: 
    ```
    OpenOCD -f interface/stlink.cfg -f target/stm32f4x.cfg
    ```
- then run GDB. Following is command executed in root of project:
    ```
    arm-none-eabi-gdb -q .\target\thumbv7em-none-eabihf\release\stm32f411-rust-neopixel
    ```

This now opens GDB. It can be used to debug code, or in this case just load it and run the project.

First write te following to connect with OpenOCD:
```
target remote :3333
```

Then to load the project:
```
load
```

And at this point, if you just want to run it:
```
continue
```

Now the onboard led should turn on.


### Other

I hope that this project and guide helps anyone with running rust on blackpill. It is really not that complicated, as it might seem on the first glance, since the whole process is nicely explained in the book and probe-run makes thing even simpler. I'm also open to any suggestions about the code and guide :)
