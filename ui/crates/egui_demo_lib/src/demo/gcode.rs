use egui::*;

#[derive(PartialEq, Default, Debug)]
pub struct Gcode {}

#[derive(Debug, PartialEq)]
pub enum GCode {
    G0 {
        // Rapid move
        x: Option<f32>,
        // The position to move to on the X axis
        y: Option<f32>,
        // The position to move to on the Y axis
        z: Option<f32>,
        // The position to move to on the Z axis
        e: Option<f32>,
        // The amount to extrude between the starting point and ending point
        f: Option<f32>, // The feedrate per minute of the move between the starting point and ending point (if supplied)
    },
    G1 {
        // Controlled move
        x: Option<f32>,
        // The position to move to on the X axis
        y: Option<f32>,
        // The position to move to on the Y axis
        z: Option<f32>,
        // The position to move to on the Z axis
        e: Option<f32>,
        // The amount to extrude between the starting point and ending point
        f: Option<f32>, // The feedrate per minute of the move between the starting point and ending point (if supplied)
    },
    G2 {
        // Controlled arc move clockwise
        x: Option<f32>,
        // The position to move to on the X axis
        y: Option<f32>,
        // The position to move to on the Y axis
        i: Option<f32>,
        // The point in X space from the current X position to maintain a constant distance from
        j: Option<f32>,
        // The point in Y space from the current Y position to maintain a constant distance from
        e: Option<f32>,
        // The amount to extrude between the starting point and ending point
        f: Option<f32>, // The feedrate per minute of the move between the starting point and ending point (if supplied)
    },
    G3 {
        // Controlled arc move counter-clockwise
        x: Option<f32>,
        // The position to move to on the X axis
        y: Option<f32>,
        // The position to move to on the Y axis
        i: Option<f32>,
        // The point in X space from the current X position to maintain a constant distance from
        j: Option<f32>,
        // The point in Y space from the current Y position to maintain a constant distance from
        e: Option<f32>,
        // The amount to extrude between the starting point and ending point
        f: Option<f32>, // The feedrate per minute of the move between the starting point and ending point (if supplied)
    },
    G4 {
        // Dwell
        p: Option<u32>,
        // Time to wait, in milliseconds
        s: Option<u32>, // Time to wait, in seconds
    },
    G6 {
        // Set coordinate system offsets
        a: Option<f32>,
        // Stepper A position or angle
        b: Option<f32>,
        // Stepper B position or angle
        c: Option<f32>,
        // Stepper C position or angle
        r: Option<bool>, // Relative move flag
    },
    G10 {
        // Retract
        l: Option<u32>,
        // Offset mode
        p: Option<u32>,
        // Tool number
        x: Option<f32>,
        // X offset
        y: Option<f32>,
        // Y offset
        z: Option<f32>,
        // Z offset
        uvwabc: Option<[Option<f32>; 6]>,
        // Other axis offsets
        r: Option<f32>,
        // Standby temperature(s) (RepRapFirmware)
        s: Option<f32>, // Active temperature(s) (RepRapFirmware)
    },
    G11 {
        // Unretract
        s: Option<u32>, // Retract length (S1 = long retract, S0 = short retract = default) (Repetier only)
    },
    G12 {
        // Clean tool
        p: Option<u32>,
        // Pattern style selection
        s: Option<u32>,
        // Number of strokes (i.e., back-and-forth movements)
        t: Option<u32>, // Number of repetitions
    },
    G17,
    // Select plane
    G18,
    // Select plane
    G19,
    // Select plane
    G20,
    // Set units to inches
    G21,
    // Set units to mm
    G22,
    // Do a retract move
    G23,
    // Do a recover move
    G26 {
        // Mesh validation pattern
        c: Option<bool>,
        // Run the test sequence
        p: Option<bool>,
        // Print a mesh validation pattern
        o: Option<f32>, // Offset for the test sequence
    },
    G27 {
        // Park toolhead
        p: Option<u32>, // Park mode (0, 1, or 2)
    },
    G28 {
        // Move to Origin (Home)
        x: bool,
        // Flag to go back to the X axis origin
        y: bool,
        // Flag to go back to the Y axis origin
        z: bool,
        // Flag to go back to the Z axis origin
        p: Option<bool>,
        // Reserved by Prusa
        i: Option<bool>, // Reserved by Prusa
    },
    G29 {
        // Detailed Z-Probe
        s: Option<i32>,
        // Firmware-dependent behavior
        p: Option<String>, // Optional file name for bed height map file (RepRapFirmware only)
    },
    G30 {
        // Single Z-Probe
        p: Option<u32>,
        // Probe point number
        x: Option<f32>,
        // X coordinate
        y: Option<f32>,
        // Y coordinate
        z: Option<f32>,
        // Z coordinate
        h: Option<f32>,
        // Height correction
        s: Option<i32>, // Set parameter
    },
    G31 {
        // Set or Report Current Probe status
        p: Option<u32>,
        // Trigger value
        x: Option<f32>,
        // Probe X offset
        y: Option<f32>,
        // Probe Y offset
        z: Option<f32>,
        // Trigger Z height
        c: Option<f32>,
        // Temperature coefficient(s) of trigger height
        s: Option<i32>,
        // Calibration temperature
        t: Option<u32>, // Z probe type to which these parameters apply
    },
    G32 {
        // Probe Z and calculate Z plane
        s: Option<u32>,
        // Bed leveling method
        p: Option<u32>, // Bed correction method
    },
    G33 {
        // Firmware dependent
        l: Option<u32>,
        // List distortion matrix in a report
        r: Option<u32>,
        // Reset distortion matrix
        x: Option<f32>,
        // X position for setting nearest point correction
        y: Option<f32>,
        // Y position for setting nearest point correction
        z: Option<f32>, // Z correction for nearest point
    },
    G34 {
        // Z Stepper Auto-Align (Marlin, RepRapFirmware) or Calculate Delta Height from toolhead position (MK4duo)
        i: Option<u32>,
        // Number of iterations (Z Stepper Auto-Align)
        t: Option<f32>,
        // Target accuracy (Z Stepper Auto-Align)
        a: Option<f32>, // Amplification (Z Stepper Auto-Align)
    },
    G38_2 {
        x: Option<f32>,
        // target X coordinate
        y: Option<f32>,
        // target Y coordinate
        z: Option<f32>,
        // target Z coordinate
        f: Option<f32>, // feedrate in mm/min
    },
    G38_3 {
        x: Option<f32>,
        // target X coordinate
        y: Option<f32>,
        // target Y coordinate
        z: Option<f32>,
        // target Z coordinate
        f: Option<f32>, // feedrate in mm/min
    },
    G38_4 {
        x: Option<f32>,
        // target X coordinate
        y: Option<f32>,
        // target Y coordinate
        z: Option<f32>,
        // target Z coordinate
        f: Option<f32>, // feedrate in mm/min
    },
    G38_5 {
        x: Option<f32>,
        // target X coordinate
        y: Option<f32>,
        // target Y coordinate
        z: Option<f32>,
        // target Z coordinate
        f: Option<f32>, // feedrate in mm/min
    },
    G40,
    // turns off cutter compensation
    G42 {
        i: Option<u32>,
        // grid X index (zero-based)
        j: Option<u32>,
        // grid Y index (zero-based)
        p: Option<bool>,
        // probe flag
        f: Option<f32>, // feedrate (mm/min)
    },
    G53,
    // coordinate system select
    G54,
    // coordinate system select
    G55,
    // coordinate system select
    G56,
    // coordinate system select
    G57,
    // coordinate system select
    G58,
    // coordinate system select
    G59,
    // coordinate system select
    G60 {
        s: Option<u32>, // memory slot # (0-based) to save into (default 0)
    },
    G68 {
        x: Option<f32>,
        // centre X coordinate to rotate about
        y: Option<f32>,
        // centre Y coordinate to rotate about
        r: Option<f32>,
        // angle to rotate in degrees
        i: Option<bool>, // if present, the R parameter is added to the existing rotation instead of being absolute
    },
    G69,
    // cancel coordinate rotation
    G75,
    // print PINDA temperature interpolating
    G76 {
        b: Option<bool>,
        // calibrate bed only
        p: Option<bool>, // calibrate probe only
    },
    G80,
    // Cancel Canned Cycle (CNC specific)
    G81,
    // Mesh bed leveling status
    G82,
    // Single Z probe at current location
    G83,
    // Babystep in Z and store to EEPROM
    G84,
    // UNDO Babystep Z (move Z axis back)
    G85,
    // Pick best babystep
    G86,
    // Disable babystep correction after home
    G87,
    // Enable babystep correction after home
    G88,
    // Reserved
    G90,
    // Set to Absolute Positioning
    G91,
    // Set to Relative Positioning
    G92 {
        // Set Position
        x: Option<f32>,
        // new X axis position
        y: Option<f32>,
        // new Y axis position
        z: Option<f32>,
        // new Z axis position
        e: Option<f32>, // new extruder position
    },
    G93,
    // Feed Rate Mode (Inverse Time Mode) (CNC specific)
    G94,
    // Feed Rate Mode (Units per Minute) (CNC specific)
    G98,
    // Activate farm mode
    G99,
    // Deactivate farm mode
    G100 {
        // Calibrate floor or rod radius
        x: Option<bool>,
        // Flag to set floor for X axis
        y: Option<bool>,
        // Flag to set floor for Y axis
        z: Option<bool>,
        // Flag to set floor for Z axis
        r: Option<f32>, // Radius to add
    },
    G130 {
        // Set digital potentiometer value
        x: Option<u8>,
        y: Option<u8>,
        z: Option<u8>,
        a: Option<u8>,
        b: Option<u8>,
    },
    G161 {
        // Home axes to minimum
        x: Option<bool>,
        y: Option<bool>,
        z: Option<bool>,
        f: Option<f32>, // Desired feedrate for this command
    },
    G162 {
        // Home axes to maximum
        x: Option<bool>,
        y: Option<bool>,
        z: Option<bool>,
        f: Option<f32>, // Desired feedrate for this command
    },
    G425 {
        // Perform auto-calibration with calibration cube
        b: Option<bool>,
        // Perform calibration of backlash only
        t: Option<u32>,
        // Perform calibration of toolhead only
        v: Option<bool>,
        // Probe cube and print position, error, backlash and hotend offset
        u: Option<f32>, // Uncertainty, how far to start probe away from the cube (mm)
    },
    M0 {
        // Stop or Unconditional stop
        p: Option<u32>,
        // Time to wait, in milliseconds
        s: Option<u32>, // Time to wait, in seconds
    },
    M1 {
        // Sleep or Conditional stop
        message: Option<String>, // Optional message to display
    },
    M2 {},
    // Program End
    M3 {
        // Spindle On, Clockwise (CNC specific)
        s: Option<u32>, // Spindle RPM
    },
    M4 {
        // Spindle On, Counter-Clockwise (CNC specific)
        s: Option<u32>, // Spindle RPM
    },
    M5 {},
    // Spindle Off (CNC specific)
    M6 {},
    // Tool change
    M7 {},
    // Mist Coolant On (CNC specific)
    M8 {},
    // Flood Coolant On (CNC specific)
    M9 {},
    // Coolant Off (CNC specific)
    M10 {},
    // Vacuum On (CNC specific)
    M11 {},
    // Vacuum Off (CNC specific)
    M13 {},
    // Spindle on (clockwise rotation) and coolant on (flood)
    M16 {
        // Expected Printer Check
        machine_name: String, // The expected machine name
    },
    M17 {
        // Enable/Power all stepper motors
        x: Option<bool>,
        // Enable X axis stepper motor
        y: Option<bool>,
        // Enable Y axis stepper motor
        z: Option<bool>,
        // Enable Z axis stepper motor
        e: Option<bool>, // Enable all extruder stepper motors
    },
    M18 {
        // Disable all stepper motors
        x: Option<bool>,
        // Disable X axis stepper motor
        y: Option<bool>,
        // Disable Y axis stepper motor
        z: Option<bool>,
        // Disable Z axis stepper motor
        e: Option<bool>,
        // Disable all extruder stepper motors
        s: Option<u32>, // Idle timeout in seconds
    },
    M20 {
        // M20: List SD card
        s: Option<u32>,
        // Output style
        r: Option<u32>,
        // File number to start at
        p: Option<String>,
        // Directory to list
        l: Option<()>,
        // Reports long filenames instead of just short filenames
        t: Option<()>, // Report timestamps as well
    },
    M21 {
        // M21: Initialize SD card
        p: Option<u32>, // SD card number (RepRapFirmware only, default 0)
    },
    M22 {
        // Release SD card
        p: Option<u32>, // SD card number (RepRapFirmware only, default 0)
    },
    M23 {
        // Select SD file
        filename: String, // The file to select for printing
    },
    M24,
    // Start/resume SD print
    M25,
    // Pause SD print
    M26 {
        // Set SD position
        s: Option<u32>,
        // File position from start of file in bytes
        p: Option<f32>, // (Optional, RepRapFirmware only) Proportion of the first move to be skipped, default 0.0, must be less than 1.0
    },
    M27 {
        // Report SD print status
        c: Option<bool>,
        // Report the open file's name and long name (Marlin 1.1.9 and up)
        s: Option<u32>, // Set the auto-report interval (Marlin 1.1.9 and up)
    },
    M28 {
        // Begin write to SD card
        filename: String, // The file to create (or overwrite) on the SD card
    },
    M29 {
        // Stop writing to SD card
        filename: String, // The file opened by M28 command
    },
    M30 {
        // Delete a file on the SD card
        filename: String, // The file to delete from the SD card
    },
    M31,
    // Output time since last M109 or SD card start to serial
    M32 {
        // Select file and start SD print
        filename: String, // The file to select and start printing
    },
    M33 {
        // Get the long name for an SD card file or folder
        path: String, // The DOS path to get the long name for
    },
    M34 {
        // Set SD file sorting options
    },
    M35 {
        // Upload firmware NEXTION from SD
    },
    M36 {
        // Return file information
        filename: Option<String>,
    },
    M36_1 {
        // Return embedded thumbnail data
        p: Option<String>,
        // Name of the GCode file from which thumbnail data is to be retrieved
        s: Option<u32>, // Byte offset into the file at which thumbnail data is to be fetched
    },
    M37 {
        // Simulation mode
        s: Option<u8>,
        // Enter or leave simulation mode
        p: Option<String>, // (optional) Simulate printing a file from SD card
    },
    M38 {
        // Compute SHA1 hash of target file
        target_file: String,
    },
    M39 {
        // Report SD card information
        p: Option<u8>,
        // SD slot number, default 0
        s: Option<u8>, // Response format. S0 returns a plain text response, S2 returns a response in JSON format.
    },
    M40 {
        // Eject
    },
    M41 {
        // Loop
    },
    M42 {
        // Switch I/O pin
        p: u32,
        // Pin number
        s: u32, // Pin value
    },
    M43MaterialExhausted,
    // M43: Stand by on material exhausted
    M43PinReport {
        // M43: Pin report and debug
        e: Option<bool>,
        // Enable / disable background endstop monitoring
        p: Option<u32>,
        // Pin to read or watch. If omitted, read/watch all pins
        w: Option<bool>,
        // bool watch pins -reporting changes- until reset, click, or M108
        i: Option<bool>, // bool Flag to ignore pin protection
    },
    M44CodesDebug {
        // M44: Codes debug - report codes available
        i: Option<String>,
        // G-code list
        j: Option<String>, // M-code list
    },
    M44ResetBedCalibration,
    // M44: Reset the bed skew and offset calibration
    M45 {
        // M45: Bed skew and offset with manual Z up
        v: Option<u32>, // Verbosity level 1, 10 and 20 (low, mid, high). Only when SUPPORT_VERBOSITY is defined. This parameter is optional.
    },
    M46,
    // M46: Show the assigned IP address
    M47,
    // M47: Show end stops dialog on the display
    M48 {
        // M48: Measure Z-Probe repeatability
        p: Option<u32>,
        // number of points
        x: Option<f32>,
        // position on the X axis
        y: Option<f32>,
        // position on the Y axis
        v: Option<u32>,
        // verbosity
        e: Option<bool>,
        // engage
        l: Option<u32>,
        // legs of travel
        s: Option<bool>, // schizoid
    },
    M49 {
        // M49: Set G26 debug flag
        s: Option<bool>, // Enable G26 verbose debug output
    },
    M70 {
        // M70: Display message
        p: Option<u32>,
        // The time to display the message for
        message: Option<String>, // The message to display
    },
    M72 {
        // M72: Play a tone or song
        p: Option<u32>, // The ID of the song to play
    },
    M73B {
        // M73: Set/Get build percentage
        p: Option<u32>,
        // Percent in normal mode
        r: Option<u32>,
        // Time remaining in normal mode (minutes)
        q: Option<u32>,
        // Percent in silent mode
        s: Option<u32>,
        // Time remaining in silent mode (minutes)
        c: Option<u32>,
        // Time to change/pause/user interaction in normal mode (minutes)
        d: Option<u32>, // Time to change/pause/user interaction in silent mode (minutes)
    },
    M75,
    // M75: Start the print job timer
    M76,
    // M76: Pause the print job
    M80 {
        // ATX Power On
        c: Option<String>, // Name of the pin used to control the power supply, default "pson"
    },
    M81 {
        // ATX Power Off
        p: Option<bool>,
        // Quit the daemon (redeem only)
        r: Option<bool>,
        // Restart the daemon (redeem only)
        s: Option<u8>, // Power off immediately (default) or when all thermostatic fans have turned off
    },
    M82,
    // Set extruder to absolute mode
    M83,
    // Set extruder to relative mode
    M84 {
        // Stop idle hold
        i: Option<u32>, // Reset flags
    },
    M85 {
        // Set Inactivity Shutdown Timer
        s: Option<u32>, // Seconds
    },
    M86 {
        // Set Safety Timer expiration time
        s: Option<u32>, // Seconds
    },
    M87,
    // Cancel Safety Timer
    M92 {
        // Set axis_steps_per_unit
        x: Option<f32>,
        // Steps per unit for the X drive
        y: Option<f32>,
        // Steps per unit for the Y drive
        z: Option<f32>,
        // Steps per unit for the Z drive
        e: Option<f32>,
        // Steps per unit for the extruder drive(s)
        s: Option<u32>, // Defines the microstepping used for the above steps per unit
    },
    M93,
    // Send axis_steps_per_unit
    M98 {
        // Call Macro/Subprogram
        p: String, // Macro filename
    },
    M99,
    // Return from Macro/Subprogram
    M101,
    // Turn extruder 1 on (Forward), Undo Retraction
    M102 {
        // Turn extruder 1 on (Reverse) or Configure Distance Sensor
        s: Option<i32>, // Adjustable Z height (Marlin) or command (negative value)
    },
    M103,
    // Turn all extruders off, Extruder Retraction
    M104 {
        // Set Extruder Temperature
        c: Option<f32>,
        // Use fan for cooling (Only Prusa)
        d: Option<f32>,
        // Display temperature (Only Prusa)
        s: Option<f32>,
        // Target temperature
        r: Option<f32>, // Idle temperature (Only MK4duo)
    },
    M105,
    // Get Extruder Temperature
    M106 {
        // Fan On
        p: Option<u8>,
        // Fan number (optional, defaults to 0)
        s: Option<f32>,
        // Fan speed (0 to 255 or 0.0 to 1.0)
        i: Option<u8>,
        // Invert signal, or disable fan
        f: Option<u32>,
        // Set fan PWM frequency, in Hz
        l: Option<f32>,
        // Set minimum fan speed (0 to 255 or 0.0 to 1.0)
        x: Option<f32>,
        // Set maximum fan speed (0 to 255 or 0.0 to 1.0)
        b: Option<f32>,
        // Blip time
        h: Option<String>,
        // Select heaters monitored when in thermostatic mode
        r: Option<u32>,
        // Restore fan speed to the value it has when the print was paused
        t: Option<f32>,
        // Set thermostatic mode trigger temperature
        c: Option<String>, // Set custom name (RRF > 2.01 only)
    },
    M107,
    // Fan Off
    M108,
    // Cancel Heating
    M109 {
        // Set Extruder Temperature and Wait
        c: Option<bool>,
        // Use fan for cooling (Only Prusa)
        s: Option<f32>,
        // Minimum target temperature, waits until heating
        r: Option<f32>,
        // Maximum target temperature, waits until cooling (Sprinter) or accurate target temperature (Marlin and MK4duo)
        t: Option<u8>,
        // Tool number (RepRapFirmware and Klipper), optional
        f: Option<bool>, // Use extruder fan to speed up cooling (if not heating)
    },
    M110 {
        n: Option<u32>, // Line number
    },
    M111 {
        p: Option<u32>,
        // Debug module (only available in RepRapFirmware)
        s: Option<u32>, // Debug on/off
    },
    M112,
    // Full (Emergency) Stop
    M113 {
        s: Option<f32>, // Set Extruder PWM or Host Keepalive interval to set (depends on firmware)
    },
    M114,
    // Get Current Position
    M115 {
        b: Option<u32>,
        // Expansion board number (RepRapFirmware 3 only)
        p: Option<u32>,
        // Electronics type (RepRapFirmware only)
        v: Option<bool>,
        // Report the Prusa version number (Prusa Firmware only)
        u: Option<String>, // Check the firmware version provided (Prusa Firmware only)
    },
    M116 {
        // Wait
        p: Option<u32>,
        // Tool number
        h: Option<u32>,
        // Heater number
        c: Option<u32>, // Chamber number
    },
    M117_GetZeroPosition,
    // Get Zero Position
    M117_DisplayMessage {
        message: String, // Message to display on LCD
    },
    M118 {
        p: Option<u32>,
        // Message target(s): 0 = generic [default], 1 = USB, 2 = LCD, 3 = HTTP, 4 = Telnet
        s: Option<String>, // Message to send
    },
    M119,
    // Get Endstop Status
    M120,
    // Push
    M121,
    // Pop
    M122 {
        b: Option<u32>,
        // Expansion board number for which diagnostics are requested, default 0 which means main board
        p: Option<u32>, // Optional parameter to specify what diagnostics are required
    },
    M124_FirmwareDependent,
    // Firmware Dependent
    M124_ImmediateMotorStop,
    // Immediate Motor Stop
    M124_SetEndstopPullup {
        // Set Endstop Pullup
        x: Option<u8>,
        y: Option<u8>,
        z: Option<u8>,
        i: Option<u8>,
        j: Option<u8>,
        k: Option<u8>,
        p: Option<u8>,
        d: Option<u8>,
    },
    M126_OpenValve {
        // Open Valve
        p: Option<u32>,
    },
    M126_MakerBot {
        // MakerBot
        t: Option<u32>,
    },
    M127_CloseValve {
        // Close Valve
        p: Option<u32>,
    },
    M127_MakerBot {
        // MakerBot
        t: Option<u32>,
    },
    M128_ExtruderPressurePWM {
        // Extruder Pressure PWM
        s: Option<u32>,
    },
    M129_ExtruderPressureOff {
        // Extruder Pressure Off
        p: Option<u32>,
    },
    M130_SetPID_PValue {
        // Set PID P value
        p: Option<u32>,
        s: Option<f32>,
    },
    M131_SetPID_IValue {
        // Set PID I value
        p: Option<u32>,
        s: Option<f32>,
    },
    M132_SetPID_DValue {
        // Set PID D value
        p: Option<u32>,
        s: Option<f32>,
    },
    M132_MakerBot {
        // MakerBot
        x: Option<f32>,
        y: Option<f32>,
        z: Option<f32>,
        a: Option<f32>,
        b: Option<f32>,
    },
    M133_SetPID_ILimitValue {
        // Set PID I limit value
        p: Option<u32>,
        s: Option<f32>,
    },
    M133_MakerBot {
        // MakerBot
        t: Option<u32>,
        p: Option<u32>,
    },
    M134_WritePIDValuesToEEPROM,
    // Write PID values to EEPROM
    M134_MakerBot {
        // MakerBot
        t: Option<u32>,
        p: Option<u32>,
    },
    M135_SetPIDSampleInterval {
        // Set PID sample interval
        s: Option<u32>,
    },
    M135_MakerBot {
        // MakerBot
        t: Option<u32>,
    },
    M133 {
        // Wait for the toolhead to reach its target temperature
        t: Option<u32>,
        // Extruder to wait for
        p: Option<u32>, // Time limit, in seconds
    },
    M134,
    // Write PID values to EEPROM
    M135 {
        // Set PID sample interval
        s: Option<u32>, // Heat sample time in seconds
    },
    M136,
    // Print PID settings to host
    M140 {
        // Set Bed Temperature (Fast)
        p: Option<u32>,
        // Bed heater index
        h: Option<u32>,
        // Heater number
        t: Option<u32>,
        // Tool number
        s: Option<f32>,
        // Active/Target temperature
        r: Option<f32>, // Standby temperature
    },
    M141 {
        // Set Chamber Temperature (Fast)
        p: Option<u32>,
        // Chamber index
        h: Option<u32>,
        // Heater number
        t: Option<u32>,
        // Tool number
        s: Option<f32>,
        // Active/Target temperature
        r: Option<f32>, // Standby temperature
    },
    M142,
    // Firmware dependent
    M143 {
        // Maximum heater temperature
        h: Option<u32>,
        // Heater number
        s: Option<f32>, // Maximum temperature
    },
    M144 {
        // Bed Standby
        p: Option<u32>,
        // Bed heater number
        s: Option<u32>, // 0 = set bed heater to standby (default), 1 = set bed heater active
    },
    M146 {
        // Set Chamber Humidity
        r: Option<f32>, // Relative humidity in percent
    },
    M149 {
        // Set temperature units
        c: bool,
        // Flag to treat temperature as degrees Celsius
        k: bool, // Flag to treat temperature as Kelvin
    },
    M150 {
        // Set LED color
        r: Option<u32>,
        // Red component
        u: Option<u32>,
        // Green component
        b: Option<u32>,
        // Blue component
        w: Option<u32>,
        // White component (Marlin)
        p: Option<u32>, // Brightness (0-255)
    },
    M155 {
        // Automatically send temperatures
        s: Option<u32>,
        // Enable sending temperatures = 1, disable = 0
        c: Option<u8>, // Activate auto-report function (bit mask). Default is temperature
    },
    M160 {
        // Number of mixed materials
        s: Option<u32>,
    },
    M163 {
        // Set weight of mixed material
        s: Option<u32>,
        // Extruder number
        p: Option<f32>, // Weight
    },
    M164 {
        // Store weights
        s: Option<u32>,
        // Virtual extruder number
        p: Option<u32>, // Store to eeprom (P0 = no, P1 = yes)
    },
    M165 {
        // Set multiple mix weights
        a: Option<f32>,
        // Mix factor for extruder stepper 1
        b: Option<f32>,
        // Mix factor for extruder stepper 2
        c: Option<f32>,
        // Mix factor for extruder stepper 3
        d: Option<f32>,
        // Mix factor for extruder stepper 4
        h: Option<f32>,
        // Mix factor for extruder stepper 5
        i: Option<f32>, // Mix factor for extruder stepper 6
    },
    M190 {
        // Wait for bed temperature to reach target temp
        s: Option<f32>,
        // Minimum target temperature, waits until heating
        r: Option<f32>, // Accurate target temperature, waits until heating and cooling (Marlin and Prusa)
    },
    M191 {
        // Wait for chamber temperature to reach target temp
        s: Option<f32>,
        // Minimum target temperature, waits until heating
        r: Option<f32>, // Accurate target temperature, waits until heating and cooling (Marlin)
    },
    M200 {
        // Set filament diameter
        d: Option<f32>,
        // Set the filament diameter in current units. If non-zero, enable Volumetric Extrusion
        t: Option<u32>,
        // Select the target extruder. If omitted, the active extruder
        s: Option<u32>,
        // Enable or Disable Volumetric Extrusion (without modifying the filament diameter)
        l: Option<f32>, // Set the Maximum Extrusion Volume in mm^3 per second. (Ignores units set by G20.) Use L0 for no limit
    },
    M201 {
        // Set max acceleration
        x: Option<u32>,
        // Acceleration for X axis in units/s^2
        y: Option<u32>,
        // Acceleration for Y axis in units/s^2
        z: Option<u32>,
        // Acceleration for Z axis in units/s^2
        e: Option<u32>, // Acceleration for the active or specified extruder in units/s^2
    },
    M202 {
        // Set max travel acceleration
        x: Option<f32>,
        // Maximum travel acceleration for X axis
        y: Option<f32>, // Maximum travel acceleration for Y axis
    },
    M203 {
        // Set maximum feedrate
        x: Option<f32>,
        // Maximum feedrate for X axis
        y: Option<f32>,
        // Maximum feedrate for Y axis
        z: Option<f32>,
        // Maximum feedrate for Z axis
        e: Option<f32>,
        // Maximum feedrate for extruder drives
        i: Option<f32>, // (RepRapFirmware) Minimum feed rate (optional)
    },
    M204 {
        // Set default acceleration
        p: Option<f32>,
        // Acceleration for printing moves
        t: Option<f32>, // Acceleration for travel moves
    },
    M205 {
        // Advanced settings
        s: Option<f32>,
        // Minimum travel speed
        t: Option<f32>,
        // Minimum travel speed
        b: Option<f32>,
        // Min segment time
        x: Option<f32>,
        // Max XY jerk
        z: Option<f32>,
        // Max Z jerk
        e: Option<f32>, // Max E jerk
    },
    M206 {
        // Offset axes
        x: Option<f32>,
        // X axis offset
        y: Option<f32>,
        // Y axis offset
        z: Option<f32>, // Z axis offset
    },
    M207 {
        s: Option<f32>,
        // Positive length to retract, in mm
        r: Option<f32>,
        // Positive or negative additional length to un-retract, in mm (RepRapFirmware only)
        f: Option<f32>,
        // Retraction feedrate, in mm/min
        t: Option<f32>,
        // Feedrate for un-retraction if different from retraction, mm/min (RepRapFirmware 1.16 and later only)
        z: Option<f32>, // Additional zlift/hop
    },
    M208 {
        s: Option<f32>,
        // 0 = set axis maximum (default), 1 = set axis minimum
        x: Option<f32>,
        // X axis limit
        y: Option<f32>,
        // Y axis limit
        z: Option<f32>, // Z axis limit
    },
    M209 {
        // M209: Enable automatic retract
        s: Option<u8>, // The S parameter turns Automatic Retract Detection on (1) or off (0)
    },
    M210 {
        // M210: Set homing feedrates
        x: Option<f32>,
        // The feedrate for homing on the X axis (mm per minute)
        y: Option<f32>, // The feedrate for homing on the Y axis (mm per minute)
    },
    M211 {
        // M211: Disable/Enable software endstops
        s: Option<u8>,
        // 1=enable or 0=disable the state of software endstop
        x: Option<u8>,
        // 1=max endstop or 0=min endstop for X axis
        y: Option<u8>,
        // 1=max endstop or 0=min endstop for Y axis
        z: Option<u8>, // 1=max endstop or 0=min endstop for Z axis
    },
    M212 {
        // M212: Set Bed Level Sensor Offset
        z: Option<f32>, // Set the Z home offset (mm)
    },
    M214 {
        // M214: Set Arc configuration values
        p: Option<f32>,
        // Max and default millimeters per arc segment
        s: Option<f32>,
        // Minimum allowable millimeters per arc segment
        n: Option<u32>,
        // Number of arcs to draw before correcting the small angle approximation
        r: Option<u32>,
        // Minimum number of segments per arcs of any radius
        f: Option<u32>, // Number of segments per second
    },
    M217 {
        // M217: Toolchange Parameters
        s: Option<f32>,
        // Retract length (mm)
        p: Option<f32>,
        // Prime feedrate (mm/min)
        r: Option<f32>,
        // Retract feedrate (mm/min)
        x: Option<f32>,
        // Park position/raise X (mm)
        y: Option<f32>,
        // Park position/raise Y (mm)
        z: Option<f32>, // Z raise (mm)
    },
    M218 {
        // M218: Set Hotend Offset
        t: Option<u8>,
        // Extruder number
        x: Option<f32>,
        // Offset on X axis (mm)
        y: Option<f32>, // Offset on Y axis (mm)
    },
    M220 {
        // M220: Set speed factor override percentage
        s: Option<u32>, // Speed factor override percentage (0..100 or higher)
    },
    M221 {
        // M221: Set extrude factor override percentage
        s: Option<u32>,
        // Extrude factor override percentage (0..100 or higher)
        d: Option<u32>, // Extruder drive number (optional)
    },
    M222 {
        // Set speed of fast XY moves
        speed: f32, // Speed in mm/min
    },
    M223 {
        // Set speed of fast Z moves
        speed: f32, // Speed in mm/min
    },
    M224 {
        // Enable extruder during fast moves
        enable: bool, // True to enable, false to disable
    },
    M225 {
        // Disable on extruder during fast moves
        disable: bool, // True to disable, false to enable
    },
    M226 {
        // G-code Initiated Pause
        message: Option<String>, // Optional message to display during pause
    },
    M226Pin {
        // Wait for pin state
        pin_number: f32,
        // Pin number to check
        pin_state: f32, // Pin state to wait for
    },
    M227 {
        // Enable Automatic Reverse and Prime
        steps: (f32, f32), // Tuple containing the number of steps for reversing and priming the extruder
    },
    M228 {
        // Disable Automatic Reverse and Prime
        disabled: bool, // True to disable, false to enable
    },
    M229 {
        // Enable Automatic Reverse and Prime
        extruder_params: (f32, f32), // Tuple containing extruder screw rotations for reversing and priming
    },
    M230 {
        // Disable / Enable Wait for Temperature Change
        enable: bool, // True to enable, false to disable
    },
    M231 {
        // Set OPS parameter
        ops_mode: f32,
        // Mode for operations
        min_distance: f32,
        // Minimum distance
        retract: f32,
        // Retraction value
        backslash: f32,
        // Backslash value
        retract_move: f32, // Retraction move value
    },
    M232,
    // Read and reset max. advance values
    M240 {
        // Trigger camera
        message: Option<String>, // Optional message to display
    },
    M240Belt {
        // Start conveyor belt motor / Echo off
        disabled: bool, // True to disable, false to enable
    },
    M241 {
        // Stop conveyor belt motor / echo on
        enabled: bool, // True to enable, false to disable
    },
    M245,
    // Start cooler
    M246,
    // Stop cooler
    M250 {
        // Set LCD contrast
        c: i32, // contrast value (0..63)
    },
    M256 {
        // Set LCD brightness
        b: i32, // brightness value (0..255)
    },
    M251 {
        // Measure Z steps from homing stop (Delta printers)
        s: i32, // 0 = reset, 1 = print, 2 = store to Z length
    },
    M260 {
        // i2c Send Data
        a: i32,
        // I2C address (0-127)
        bytes: Vec<u8>, // Bytes to send (up to 32)
    },
    M261 {
        // i2c Request Data
        a: i32,
        // I2C address
        b: i32,
        // Number of bytes to request
        s: i32, // Style of output (0=raw, 1=hex, 2=int, 3=decimal)
    },
    M280 {
        // Set servo position
        p: i32,
        // Servo index
        s: f32,
        // Angle or microseconds
        i: Option<bool>, // Invert polarity
    },
    M281 {
        // Set Servo Angles
        p: i32,
        // Servo index
        angle_1: f32,
        // Angle for servo 1
        angle_2: Option<f32>,
        // Angle for servo 2 (optional)
        angle_3: Option<f32>,
        // Angle for servo 3 (optional)
        angle_4: Option<f32>,
        // Angle for servo 4 (optional)
        angle_5: Option<f32>,
        // Angle for servo 5 (optional)
        angle_6: Option<f32>,
        // Angle for servo 6 (optional)
        angle_7: Option<f32>,
        // Angle for servo 7 (optional)
        angle_8: Option<f32>, // Angle for servo 8 (optional)
    },
    M292 {
        // Acknowledge message
        p: Option<i32>, // Whether the current operation shall be cancelled. Only legal if M291 was called with S=3
    },
    M300 {
        // Play beep sound
        s: Option<i32>,
        // Frequency in Hz
        p: Option<i32>,
        // Duration in milliseconds
        v: Option<f32>, // Volume in range 0 - 1
    },
    M301 {
        // Set PID parameters
        h: i32,
        // Heater number
        p: Option<f32>,
        // Proportional (Kp)
        i: Option<f32>,
        // Integral (Ki)
        d: Option<f32>, // Derivative (Kd)
    },
    M302 {
        // Allow cold extrudes
        s: Option<i32>,
        // Cold extrude minimum temperature
        p: Option<i32>,
        // Cold extrude allow state
        r: Option<i32>, // Cold retraction minimum temperature
    },
    M303 {
        // Run PID tuning
        s: Option<i32>,
        // Temperature (used for hot end or bed)
        c: Option<i32>,
        // Cycles (used for bed)
        h: Option<i32>,
        // Heater number (used in RepRapFirmware)
        p: Option<f32>, // PWM (used in RepRapFirmware)
    },
    M304 {
        // Set PID parameters - Bed
        p: Option<f32>,
        // Proportional (Kp)
        i: Option<f32>,
        // Integral (Ki)
        d: Option<f32>, // Derivative (Kd)
    },
    M305 {
        // Set thermistor and ADC parameters
        p: u32,
        // Heater number, or virtual heater number
        s: Option<String>,
        // Heater name (optional, RepRapFirmware only)
        t: Option<u32>,
        // Thermistor resistance at 25°C (for thermistor sensors)
        c: Option<u32>,
        // Steinhart-Hart C coefficient (MK4duo and RepRapFirmware 1.17 and later), default 0
        b: Option<u32>,
        // Beta value, or the reciprocal of the Steinhart-Hart thermistor model B coefficient
        r: Option<u32>,
        // Series resistor value
        l: Option<u32>,
        // ADC low offset correction, default 0
        h: Option<u32>,
        // ADC high offset correction, default 0
        x: Option<u32>,
        // Heater ADC channel, or thermocouple or PT100 or current loop adapter channel, defaults to the same value as the P parameter
        f: Option<u32>, // Local mains frequency. Readings will be timed to optimize rejection of interference at this frequency (if the sensor interface uses a MAX31856 thermocouple chip or MAX31865 PT100 chip)
    },
    M306 {
        // Set home offset calculated from toolhead position
        z: Option<f32>, // The value specified is added to the calculated endstop position when the axes are referenced. The calculated value is derived from the distance of the toolhead from the current axis zero point.
    },
    M307 {
        // M307: Set or report heating process parameters
        h: u32,
        // Heater number (0 is usually the bed heater)
        a: Option<f32>,
        // gAin, expressed as ultimate temperature rise obtained in °C divided by the PWM fraction.
        c: Option<f32>,
        // Dominant time constant of the heating process in seconds
        d: Option<f32>,
        // Dead time in seconds
        f: Option<u32>,
        // PWM frequency to use (not supported in RepRapFirmware 3, use M950 instead)
        b: Option<u32>,
        // Selects Bang-bang control instead of PID if non-zero. Default at power-up is 0 for extruder heaters, 1 for bed and chamber heaters.
        s: Option<f32>,
        // Maximum PWM to be used with this heater on a scale of 0 to 1. Default 1.0.
        v: Option<f32>, // VIN supply voltage at which the A parameter was calibrated (RepRapFirmware 1.20 and later).
    },
    M308 {
        // Set or report sensor parameters
        s: u32,
        // Sensor number
        p: Option<String>,
        // The name of the control board pin that this sensor uses. For thermistors it is the thermistor input pin name. For sensors connected to the SPI bus it is the name of the output pin used as the chip select.
        y: Option<String>,
        // The sensor and interface type, e.g. "thermistor", "pt1000", "rtdmax31865", "max31855", "max31856", "linear-analog", "dht22-temp", "dht22-humidity", "current-loop-pyro"
        a: Option<String>, // Sensor name (optional), displayed in the web interface
    },
    M309 {
        // Set or report heater feedforward
        p: Option<u32>,
        // Tool number
        s: Option<Vec<f32>>, // Feedforward coefficients. The number of coefficients provided must equal the number of heaters configured for the tool when it was created (see M563).
    },
    M310 {
        // Temperature model settings
        a: bool,
        // Autotune C+R values
        f: bool,
        // Force model self-test state (0=off 1=on) during autotune using current values
        s: Option<u32>,
        // Set 0=disable 1=enable
        i: Option<u32>,
        // Resistance index position (0-15)
        r: Option<f32>,
        // Resistance value at index (K/W; requires I)
        p: Option<f32>,
        // Power (W)
        c: Option<f32>,
        // Capacitance (J/K)
        b: Option<u32>,
        // Beep and warn when reaching warning threshold 0=disable 1=enable (default: 1)
        e: Option<f32>,
        // Error threshold (K/s; default in variant)
        w: Option<f32>,
        // Warning threshold (K/s; default in variant)
        t: Option<f32>, // Ambient temperature correction (K; default in variant)
    },
    M320 {
        // Activate autolevel (Repetier)
        s: Option<u32>, // If greater than 0, activate and store persistently in EEPROM
    },
    M321 {
        // Deactivate autolevel (Repetier)
        s: Option<u32>, // If greater than 0, deactivate and store persistently in EEPROM
    },
    M322 {
        // Reset autolevel matrix (Repetier)
        s: Option<u32>, // If greater than 0, also reset the matrix values saved EEPROM
    },
    M323 {
        // Distortion correction on/off (Repetier)
        s: Option<u32>,
        // 0 (disable correction) or 1 (enable correction)
        p: Option<u32>, // 1 (store correction state persistently in EEPROM)
    },
    M340 {
        // Control the servos
        p: u8,
        // Servo ID (0..3)
        s: u16, // Pulse width (500..2500)
    },
    M350 {
        // Set microstepping mode
        s: Option<u16>,
        // Stepping mode for all drivers (not supported by RepRapFirmware)
        x: Option<u16>,
        // Stepping mode for the X axis
        y: Option<u16>,
        // Stepping mode for the Y axis
        z: Option<u16>,
        // Stepping mode for the Z axis
        e: Option<String>,
        // Stepping mode for extruder(s) (for RepRapFirmware use "Enn:nn:nn" etc. for multiple extruders)
        b: Option<u16>,
        // Stepping mode for extruder 1 (not supported by RepRapFirmware, see above)
        i: Option<u8>, // Enable (1) or disable (0) microstep interpolation mode for the specified drivers, if they support it (RepRapFirmware only)
    },
    M351,
    // Toggle MS1 MS2 pins directly
    M355 {
        // Turn case lights on/off
        s: Option<u8>,
        // Enable (1) or disable (0) lights
        p: Option<u8>, // Set light power (0..255)
    },
    M360 {
        // Report firmware configuration or calibration
        // Move to Theta 0 degree position (Marlin, Smoothieware)
        // Take current position as parallel to the platform edge, and store the offset in the homing trim offset (M666) (Smoothieware only)
        p: Option<u8>, // Parameter value (0)
    },
    M361 {
        // Move to Theta 90 degree position (Marlin, Smoothieware)
        // Accept current position as 90deg to platform edge. New steps per angle is calculated and entered into memory (M92) (Smoothieware only)
        p: Option<u8>, // Parameter value (0)
    },
    M362,
    // Move to Psi 0 degree position
    M363,
    // Move to Psi 90 degree position
    M364 {
        // Move to Psi + Theta 90 degree position
        p: Option<u8>, // Extension of the grid file (Smoothieware only)
    },
    M365 {
        // SCARA scaling factor
        s: Option<f32>, // Scaling factor (default 1)
    },
    M366,
    // SCARA convert trim
    M370 {
        // Morgan manual bed level - clear map
        x: Option<u8>,
        // Divisions along X (default 5)
        y: Option<u8>, // Divisions along Y (default 5)
    },
    M371,
    // Move to next calibration position
    M372,
    // Record calibration value, and move to next position
    M373,
    // End bed level calibration mode
    M374 {
        // Save calibration grid
        extension: Option<String>,
        // Extension of the grid file (Smoothieware only)
        filename: Option<String>,
        // Name of the file to save to (RepRapFirmware only)
        z: bool, // Also save the M206 Z homing offset into the grid file (Smoothieware only)
    },
    M375 {
        // Display matrix / Load Matrix
        extension: Option<String>,
        // Extension of the grid file (Smoothieware only)
        filename: Option<String>, // Name of the file to load (RepRapFirmware only)
    },
    M376 {
        // Set bed compensation taper
        h: Option<f32>, // Height (mm) over which to taper off the bed compensation
    },
    M380,
    // Activate solenoid
    M381,
    // Disable all solenoids
    M400,
    // Wait for current moves to finish
    M401 {
        // Deploy Z Probe
        p: Option<u8>,
        // Probe number (default 0)
        s: Option<bool>,
        // Set BLTouch HS Mode (Marlin 2.0.9.3+ with BLTOUCH enabled.)
        h: bool, // Report current BLTouch HS Mode (Marlin 2.0.9.4+ with BLTOUCH enabled.)
    },
    M402 {
        // Stow Z Probe
        p: Option<u8>, // Probe number (default 0)
    },
    M403 {
        // Set filament type (material) for particular extruder and notify the MMU
        e: u8,
        // Extruder number
        f: String, // Filament type
    },
    M404 {
        // Filament width and nozzle diameter
        n: Option<f32>,
        // Nozzle diameter (mm)
        w: Option<f32>, // Filament diameter (mm)
    },
    M405 {
        // Filament Sensor on
        delay: Option<f32>, // Delay in centimeters between sensor and extruder
    },
    M406,
    // Filament Sensor off
    M407,
    // Display filament diameter
    M408 {
        response_type: u32,
        // The desired type of the JSON-style response
        sequence_num: Option<u32>, // Response sequence number
    },
    M409 {
        key: Option<String>,
        // Key string for Object Model variables
        flags: Option<String>, // Flags string for Object Model variables
    },
    M410,
    // Quick-Stop
    M412 {
        s: Option<bool>, // Enable or disable filament runout detection
    },
    M413 {
        s: Option<bool>, // Enable or disable power-loss recovery
    },
    M415 {
        s: Option<bool>,
        // Enable or disable host rescue system
        z: Option<f32>, // Set Z position as if homed
    },
    M416,
    // Host tells firmware that it will lose power
    M420 {
        r: u8,
        // Red PWM (0-255)
        e: u8,
        // Green PWM (0-255)
        b: u8, // Blue PWM (0-255)
    },
    M421 {
        i: i32,
        // Index for the X-axis
        j: i32,
        // Index for the Y-axis
        z: f32,
        // Absolute value to a mesh point or offset a mesh point by a specified value
        q: Option<f32>, // Offset a mesh point by a specified value
    },
    M422 {
        s: i32,
        // Index
        x: f32,
        // X position
        y: f32, // Y position
    },
    M423 {
        r: bool,
        // Flag to reset the X-twist data to configured defaults
        x: Option<i32>,
        // Zero-based index into the X-twist data array
        z: Option<f32>,
        // An offset value to set
        a: Option<f32>,
        // Set the starting X position
        i: Option<f32>, // Set the X spacing distance
    },
    M424 {
        z: Option<f32>, // New global offset value to apply
    },
    M425 {
        f: Option<f32>,
        // Enable/disable/fade-out backlash correction (0.0 = none to 1.0 = 100%)
        s: Option<f32>,
        // Distance over which backlash correction is spread (mm)
        x: Option<f32>,
        // Set the backlash distance on X (mm; 0 to disable)
        y: Option<f32>,
        // Set the backlash distance on Y (mm; 0 to disable)
        z: Option<f32>,
        // Set the backlash distance on Z (mm; 0 to disable)
        x_meas: bool,
        // Use measured value for backlash on X (if available)
        y_meas: bool,
        // Use measured value for backlash on Y (if available)
        z_meas: bool, // Use measured value for backlash on Z (if available)
    },
    M450,
    M451,
    M452,
    M453 {
        spindle_index: Option<u8>,
        // Spindle index, defaults to 0. Duet 2 supports 4 spindles max
        spindle_pin_cw: Option<u8>,
        // Logical pin numbers used to drive the spindle motor in clockwise direction
        spindle_pin_ccw: Option<u8>,
        // Logical pin numbers used to drive the spindle motor in counter-clockwise direction
        invert: bool,
        // Invert (true) or don't invert (false) the output polarity
        spindle_rpm: Option<f32>,
        // Spindle RPM that is achieved at full PWM. Used to convert the S parameter in M3 and M4 commands to a PWM value.
        pwm_freq: Option<f32>,
        // The PWM frequency to use
        assign_spindle: Option<u8>, // Assign spindle to a tool allowing better control in DWC
    },
    M460 {
        min_temp: f32,
        // Minimum temperature at which fan starts
        max_temp: f32, // Maximum temperature at which fan runs at full speed
    },
    M470 {
        name: String, // Name of directory to create
    },
    M471 {
        source: String,
        // Name of existing file/directory
        dest: String,
        // New name of file/directory
        delete: bool, // Setting this to true will delete an existing file that matches the T parameter value
    },
    M472 {
        name: String,
        // Name of file/directory to delete
        recursive: bool, // R1 = recursive delete
    },
    M486 {
        t: Option<i32>,
        // Total number of objects on the print bed
        s: Option<i32>,
        // Indicate which object is being printed
        a: Option<String>,
        // Optional name for the object being printed
        p: Option<i32>,
        // Cancel object with index specified
        u: Option<i32>,
        // Un-cancel object with index specified
        c: bool, // Cancel the current object
    },
    M493 {
        s: Option<i32>,
        // Set the motion/shaping mode
        p: Option<bool>,
        // Enable/disable linear advance pressure control
        k: Option<f32>,
        // Set linear advance gain
        d: Option<i32>,
        // Set dynamic frequency mode
        a: Option<f32>,
        // Set static/base frequency for the X axis
        f: Option<f32>,
        // Set frequency scaling for the X axis
        b: Option<f32>,
        // Set static/base frequency for the Y axis
        h: Option<f32>, // Set frequency scaling for the Y axis
    },
    M500,
    M501 {
        s: Option<i32>, // Enable auto-save (only in RepRapFirmware)
    },
    M502,
    M503 {
        s: bool, // Output settings as G-code only (Marlin 1.1)
    },
    M504,
    M505_ClearEepromReset,
    M505_SetConfigFolder {
        name: String, // Name of folder, default path is '/sys/' if it is a relative path
    },
    M505_SetEepromValue {
        varname: String,
        // Name of the variable to be set (up to 31 characters)
        value: String, // Value of the variable to be set (up to 63 characters)
    },
    M509,
    M510,
    M511 {
        passcode: Option<String>, // Numeric passcode to try
    },
    M512 {
        // Set Passcode
        oldpass: Option<String>,
        // The current numeric passcode
        newpass: Option<String>, // A new numeric passcode
    },
    M513 {
        // Remove Password
        currpass: Option<String>,
        // The current password (if known)
        removal_hash: Option<String>, // 24 char."PW Removal HASH String", generated from MCU PUID.
    },
    M524,
    // Abort SD Printing
    M530 {
        // Enable printing mode
        state: bool,
        // true if print has started, false if print has ended
        layers: u32, // The number of layers
    },
    M531(String),
    // Set print name
    M532 {
        // Set print progress
        progress: f32,
        // The progress percentage (0 to 100)
        layer: u32, // The currently printed layer
    },
    M540(String),
    // Set MAC address
    M544 {
        // Gcode Parser Options
        case_insensitive: bool, // true if the parser should be case insensitive, false if it should be case sensitive
    },
    M550(String),
    // Set Name
    M551(String),
    // Set Password
    M552 {
        // Set IP address, enable/disable network interface
        net_interface: Option<u32>,
        // Number of the network interface to manage (defaults to 0)
        ip_address: Option<String>,
        // IP address, 0.0.0.0 means acquire an IP address using DHCP
        enable_state: Option<i32>, // -1 = reset network interface, 0 = disable networking, 1 = enable networking as a client, 2 = enable networking as an access point (WiFi-enabled electronics only)
    },
    M553 {
        // Set Netmask
        net_interface: Option<u32>,
        // Number of the network interface to manage (defaults to 0)
        net_mask: Option<String>, // Net mask
    },
    M554 {
        // Set Gateway and/or DNS server
        net_interface: Option<u32>,
        // Number of the network interface to manage (defaults to 0)
        gateway: Option<String>,
        // Gateway address
        dns: Option<String>, // DNS address
    },
    M555 {
        p: i32, // Emulation type
    },
    M556 {
        s: f32,
        // Height of the measured distances
        x: f32,
        // Deviation in X direction
        y: f32,
        // Deviation in Y direction
        z: f32,
        // Deviation in Z direction
        p: i32, // Apply XY compensation to Y axis instead of X (defaults to 0, requires RRF 3.2-b4 or newer)
    },
    M557ProbePoint {
        p: i32,
        // Probe point number
        x: f32,
        // X coordinate
        y: f32, // Y coordinate
    },
    M557ProbeGrid {
        x: f32,
        // Minimum and maximum X coordinates to probe
        y: f32,
        // Minimum and maximum Y coordinates to probe
        r: f32,
        // Radius to probe
        s: f32,
        // Probe point spacing
        p: f32, // Number of probe points in each direction (RepRapFirmware 2.02 and later) - use instead of specifying the spacing
    },
    M558 {
        p: i32,
        // Z probe type
        f: f32,
        // Feed rate (i.e. probing speed, mm/min)
        h: f32,
        // Dive height (mm)
        i: i32,
        // Invert (I1) or do not invert (I0) the Z probe reading (RepRapFirmware 1.16 and later)
        r: f32,
        // Z probe recovery time after triggering, default zero (seconds) (RepRapFirmware 1.17 and later)
        t: f32,
        // Travel speed to and between probe points (mm/min)
        a: i32,
        // Maximum number of times to probe each point, default 1 (RepRapFirmware 1.21 and later)
        s: f32,
        // Tolerance when probing multiple times, default 0.03 (RepRapFirmware 1.21 and later)
        b: i32, // B1 turns off all heaters during probing moves and during the probe recovery time (RepRapFirmware 1.21 and later)
    },
    M559,
    M560,
    M561,
    M562 {
        p: i32, // Heater number
    },
    M563 {
        p: i32,
        // Tool number
        s: Option<String>,
        // Tool name (optional)
        d: Option<i32>,
        // Extruder drive(s)
        h: Option<i32>,
        // Heater(s)
        f: Option<i32>,
        // Fan(s) to map the print cooling fan to (RepRapFirmware 1.16 and later)
        x: Option<String>,
        // Axis or axes to map X movement to (RepRapFirmware 1.16 and later)
        y: Option<String>,
        // Axis or axes to map Y movement to (RepRapFirmware 1.16 and later)
        z: Option<String>,
        // Axis or axes to map Z movement to (RepRapFirmware 1.16 and later)
        u: Option<String>, // Axis or axes to map U movement to (RepRapFirmware 3.0 and later)
    },
    M564 {
        h: Option<u32>,
        // forbid movement of axes that have not been homed (1) or allow (0)
        s: Option<u32>, // limit movement within axis boundaries (1) or allow movement outside boundaries (0)
    },
    M565 {
        x: Option<f32>,
        // delta between the extruder and the actual trigger position of the probe on the X axis
        y: Option<f32>,
        // delta between the extruder and the actual trigger position of the probe on the Y axis
        z: Option<f32>, // delta between the extruder and the actual trigger position of the probe on the Z axis
    },
    M566 {
        x: Option<f32>,
        // Maximum instantaneous speed change of the X axis (mm/min)
        y: Option<f32>,
        // Maximum instantaneous speed change of the Y axis
        z: Option<f32>,
        // Maximum instantaneous speed change of the Z axis
        e: Option<f32>, // Maximum instantaneous speed change of the extruder drives
    },
    M567 {
        p: u32,
        // Tool number
        e: Vec<f32>, // Mix ratios
    },
    M568 {
        p: Option<u32>,
        // Tool number. If this parameter is not provided, the current tool is assumed.
        r: Option<f32>,
        // Standby temperature(s)
        s: Option<f32>,
        // Active temperature(s)
        f: Option<f32>,
        // Spindle RPM, always positive
        a: Option<u32>, // Required heater state: 0 = off, 1 = standby temperature(s), 2 = active temperature(s)
    },
    M569 {
        // Stepper driver control
        p: u32,
        // Motor driver number
        s: Option<u32>,
        // Direction of movement of the motor(s) attached to this driver: 0 = backwards, 1 = forwards (default 1)
        r: Option<u32>,
        // Driver enable polarity: 0 = active low, 1 = active high (default 0)
        t: Option<u32>,
        // Minimum driver step pulse width and interval in microseconds (RepRapFirmware 1.14 and later)
        ta: Option<(u32, u32, u32, u32)>,
        // Minimum driver step pulse width, step pulse interval, direction-to-step setup time and step-to-direction hold time, in microseconds (RepRapFirmware 1.21 and later)
        d: Option<u32>,
        // Stepper driver mode (RepRapFirmware 2.0 and later): 0=constant off time, 1=random off time, 2=spread cycle, 3=stealthChop, 4=closed loop
        f_n: Option<u32>,
        // (firmware 2.02 and later) Off-time in the chopper control register, 1 to 15
        b_n: Option<u32>,
        // (firmware 2.02 and later) Blanking time (tbl) in the chopper control register, 0 to 3. See the TMC driver datasheet.
        y: Option<(u32, u32, Option<u32>)>,
        // (firmware 2.02 and later) Hysteresis start, end and decrement values in the chopper control register. See the TMC driver datasheet for the meaning.
        c: Option<u32>,
        // Custom chopper control register value (RepRapFirmware 2.0 and later). '''Do not change this value without having a good understanding of the stepper driver driver chip!'''
        h: Option<u32>,
        // (firmware 2.02 and later) t_high parameter for those stepper driver chips that support it (e.g. TMC2208, 2224). Send M569 P# (where # is the driver number) with no additional parameters to see how this translates into mm/sec. See also the V parameter.
        v: Option<u32>, // (firmware 2.02 and later) tpwmthrs parameter for those stepper driver chips that support it (e.g. TMC2208, 2224). This is the interval in clock cycles between 1/256 microsteps below which the drivers will switch from stealthChop to to spreadCycle mode. Only applies when the driver is configured in stealthChop mode. Typical value are from 100 (high speed) to 4000 (low speed). Send M569 P# (where # is the driver number) with no additional parameters to see how this translates into axis speed in mm
    },
    M570 { // Configure heater fault detection
        s: Option<i32>, // Heater timeout (in seconds)
        h: Option<i32>, // Heater number
        p: Option<i32>, // Time in seconds for which a temperature anomaly must persist on this heater before raising a heater fault (default 5 seconds)
        t: Option<i32>, // Permitted temperature excursion from the setpoint for this heater (default 10C)
    },
    M571 { // Set output on extrude
        s: Option<f32>, // Output value
        f: Option<i32>, // Output PWM frequency (RepRapFirmware 1.17 and later)
        p: Option<i32>, // Logical pin number (RepRapFirmware 1.17 and later), defaults to the FAN0 output until M571 with a P parameter has been seen
    },
    M572 { // Set or report extruder pressure advance
        d: Option<i32>, // Extruder number
        s: Option<f32>, // Pressure advance amount (in seconds)
    },
    M573 { // Report heater PWM
        p: Option<i32>, // Heater number
    },
    M574 { // Set endstop configuration
        x: Option<i32>, // Switch position for X axis
        y: Option<i32>, // Switch position for Y axis
        z: Option<i32>, // Switch position for Z axis
        s: Option<i32>, // Endstop type: 0 = active low endstop input, 1 = active high endstop input, 2 = Z probe, 3 = motor load detection
    },
    M575 { // Set serial comms parameters
        p: Option<i32>, // Serial channel number
        b: Option<i32>, // Baud rate (optional)
        s: Option<i32>, // Protocol (optional)
    },
    M576 { // Set SPI comms parameters
        s: Option<i32>, // Maximum delay between full SPI transfers (in ms, defaults to 25ms)
        f: Option<i32>, // Maximum delay between full SPI transfers when a file is open (in ms, defaults to 5ms)
        p: Option<i32>, // Number of events required to skip the delay (defaults to 4)
    },
    M577 { // Wait until endstop is triggered
        s: Option<i32>, // Desired endstop level
        x: Option<i32>, // Select X axis endstop
        y: Option<i32>, // Select Y axis endstop
        z: Option<i32>, // Select Z axis endstop
        e: Option<i32>, // Select extruder drive endstop
    },
    M578 { // Fire inkjet bits
        p: Option<i32>, // Inkjet head number
        s: Option<i32>, // Bit pattern
    },
    M579 { // Scale Cartesian axes
        x: Option<f32>, // Scale factor for the X axis
        y: Option<f32>, // Scale factor for the Y axis
        z: Option<f32>, // Scale factor for the Z axis
    },
    M580 { // Select Roland
        r: Option<u32>, // Whether Roland mode should be activated
        p: Option<String>, // Initial text to send to the Roland controller
    },
    M581 { // Configure external trigger
        t: Option<u32>, // Logical trigger number to associate the endstop input(s) with
        x: Option<bool>, // Selects X endstop input to monitor
        y: Option<bool>, // Selects Y endstop input to monitor
        z: Option<bool>, // Selects Z endstop input to monitor
        e: Option<bool>, // Selects E endstop input to monitor
        p: Option<bool>, // Reserved for future use to allow general I/O pins to cause triggers
        s: Option<i32>,  // Whether trigger occurs on a rising edge of that input (S1, default), falling edge (S0), or ignores that input (S-1). By default, all triggers ignore all inputs.
        c: Option<u32>,  // Condition: whether to trigger at any time (C0, default) or only when printing a file from SD card (C1)
    },
    M582 { // Check external trigger
        t: Option<u32>, // Trigger number to poll
    },
    M584 { // Set drive mapping
        x: Option<Vec<u32>>, // Driver number(s) for X motor(s)
        y: Option<Vec<u32>>, // Driver number(s) for Y motor(s)
        z: Option<Vec<u32>>, // Driver number(s) for Z motor(s)
        u: Option<Vec<u32>>, // Driver number(s) for U motor(s)
        v: Option<Vec<u32>>, // Driver number(s) for V motor(s)
        w: Option<Vec<u32>>, // Driver number(s) for W motor(s)
        a: Option<Vec<u32>>, // Driver number(s) for A motor(s)
        b: Option<Vec<u32>>, // Driver number(s) for B motor(s)
        c: Option<Vec<u32>>, // Driver number(s) for C motor(s)
        e: Option<Vec<u32>>, // Driver number(s) for E motor(s)
        p: Option<u32>,      // Number of visible axes, defaults to the total number of axes configured.
    },
    M585 { // Probe Tool
        x: Option<f32>, // X tool offset
        y: Option<f32>, // Y tool offset
        z: Option<f32>, // Z tool offset
    },
    M586 { // Configure network protocols
        p: Option<u32>, // Protocol: 0 = HTTP or HTTPS, 1 = FTP or SFTP, 2 = Telnet or SSH, 3 - multicast discovery protocol, 4 = MQTT
        s: Option<u32>, // 0 = disable this protocol, 1 = enable this protocol
        r: Option<u32>, // TCP port number to use for the specified protocol
        t: Option<u32>, // 0 = don't use TLS, 1 = use TLS
        c: Option<String>, // Set or reset allowed site for cross-orgin HTTP requests
    },
    M587 { // Store WiFi host network in list, or list stored networks
        s: Option<String>, // Network SSID
        p: Option<String>, // Network password
        i: Option<String>, // IP address
        j: Option<String>, // Gateway IP address
        k: Option<String>, // Netmask
        l: Option<String>, // DNS server
        c: Option<String>, // Country code for the WiFi adapter
    },
    M588 { // Forget WiFi host network
        s: String, // SSID to remove from the networks list
    },
    M589 { // Configure access point parameters
        s: String, // The SSID that the WiFi interface should use when it is commanded to run as an access point
        p: String, // The WiFi password
        i: String, // The IP address to use
    },
    M590 { // Report current tool type and index
    },
    // M591: Configure filament monitoring
    M591 {
        c: Option<u32>, // Which input the filament sensor is connected to
        d: Option<u32>, // Extruder drive number (0, 1, 2...)
        p: Option<u32>, // Type of sensor
        s: Option<u32>, // S0 = disable filament monitoring, S1 = enable filament monitoring
        r: Option<(f32, f32)>, // Allow the filament movement reported by the sensor to be between aa% and bb% of the commanded values
        e: Option<f32>, // Minimum extrusion length before a commanded/measured comparison is done, default 3mm
        l: Option<f32>, // Filament movement per complete rotation of the sense wheel or pulse in mm
    },
    // M592: Configure nonlinear extrusion
    M592 {
        d: Option<u32>, // Extruder drive number (0, 1, 2...)
        a: Option<f32>, // A coefficient in the extrusion formula, default zero
        b: Option<f32>, // B coefficient in the extrusion formula, default zero
        l: Option<f32>, // Upper limit of the nonlinear extrusion compensation, default 0.2
        t: Option<u32>, // Reserved for future use, for the temperature at which these values are valid
    },
    // M593: Configure Input Shaping
    M593 {
        p: Option<String>, // Type of input shaping to use
        f: Option<f32>, // Centre frequency of ringing to cancel in Hz
        s: Option<f32>, // Damping factor of ringing to be cancelled, default 0.1
        l: Option<f32>, // Minimum acceleration allowed, default 10mm/sec^2
        h: Option<Vec<f32>>, // Amplitudes of each impulse except the last, normally below 1.0
        t: Option<Vec<f32>>, // Durations of each impulse except the last
    },
    // M594: Enter/Leave Height Following mode
    M594 {
        p: Option<u32>, // P1 = enter height following mode, P0 = leave height following mode
    },
    // M595: Set movement queue length
    M595 {
        p: Option<u32>, // Maximum number of moves held in the movement queue
        s: Option<u32>, // Number of pre-allocated per-motor movement objects
    },
    // M596: Select movement queue number
    M596 {
        p: Option<u32>, // Movement queue number
    },
    
}

impl GCode {
    //pub fn from_string(s: &str) -> Option<Self> {
    //    let mut parts = s.split_whitespace();
    //    let code = parts.next().unwrap_or("").to_ascii_uppercase();
        //match code.as_str() {
        //    "G0" => Some(GCode::G0 {
        //        x: Self::parse_optional_value(parts.next(), "X"),
        //        y: Self::parse_optional_value(parts.next(), "Y"),
        //        z: Self::parse_optional_value(parts.next(), "Z"),
        //        e: Self::parse_optional_value(parts.next(), "E"),
        //        f: Self::parse_optional_value(parts.next(), "F"),
        //    }),
        //    "M405" => Some(GCode::M405 {
        //        delay: Self::parse_optional_value(parts.next(), "D"),
        //    }),
        //    "M406" => Some(GCode::M406),
        //    "M407" => Some(GCode::M407),
        //}
    //}
}

impl super::Demo for Gcode {
    fn name(&self) -> &'static str {
        "🗠 Gcode"
    }

    fn show(&mut self, ctx: &Context, open: &mut bool) {
        use super::View as _;
        Window::new(self.name())
            .open(open)
            .default_size(vec2(1200.0, 800.0))
            .vscroll(false)
            .show(ctx, |ui| self.ui(ui));
    }
}

impl super::View for Gcode {
    #[allow(clippy::unused_self)]
    fn ui(&mut self, ui: &mut Ui) {
        let ui_connect = ui
            .button("Connect")
            .on_hover_text("Initiate serial communications");
        if ui_connect.clicked() {}
    }
}
