pub fn print() {
    println!("            csb2json by OSA413
        Built upon Skyth's CsbEditor and SonicAudioLib
        
        Released under the MIT License
        {}

Usage:
    `csb2json <file.csb>`
    `csb2json <file.csb> --cpk <file.cpk>`

Options:
    `--cpk <path>`      include external CPK into json dump
    `-h`, `--help`      print this help
    `-v`, `--version`   print version", env!("CARGO_PKG_REPOSITORY"));
}