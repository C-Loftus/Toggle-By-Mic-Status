# Toggle By Mic Status

Many useful accessibility software programs like break timers or voice control can trigger during meetings, causing interruptions while screensharing.

This daemon solves this issue by watching the status of the default microphone. If the microphone is in use, even if it is muted, the application will be killed. Once it is no longer in use, such as when you are no longer in a call, the application will be restarted.

This program defaults to [BreakTimer](https://www.breaktimer.app) which is a useful break timer for preventing RSI, but any program can be specified as an argument.

## Limitations

Currently this program works exclusively on MacOS.

## Installation

- MacOS: `./install_macos.sh`
