# selektor

Select from available options with scripting. This is a simple GUI application that pipes the selected option back to 
the script.

## Installation

For right now you need to [install all the dependencies for Tauri development](https://tauri.app/v1/guides/getting-started/prerequisites/)
and then run `npm run tauri build`. There is a plan to provide binaries in the future.

Copy the resulting binary nested somewhere in `src-tauri/target` to a directory in your PATH. 

## Usage

Pass a JSON object with this format into `selektor`: `[{label: "Label value", value: "Program value"}]`. You can pass this 
in either through an environment variable `SELEKTOR_OPTIONS`, through a CLI argument `--options json-text`, or through STDIN.   

If you'd like to change the prompt in the text box you can do it through the environment variable `SELKTOR_PROMPT` or
through the CLI argument `--prompt "Your Prompt Here..."`. 

