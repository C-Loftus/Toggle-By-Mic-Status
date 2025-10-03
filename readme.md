# BreakTimer Microphone Daemon

[BreakTimer](https://github.com/tom-james-watson/breaktimer-app) is a useful tool for prompting you to take break while working and preventing RSI/Eyestrain. However, it can trigger these breaks during meetings, causing interruptions while screensharing.

This daemon solves this issue by watching the status of the default microphone. If the microphone is in use, even if it is muted, the application will be killed. Once it is no longer in use, such as when you are no longer in a call, the application will be restarted. 

This program defaults to BreakTimer but any program can be specified as an argument.

## Limitations

Currently this program works exclusively on MacOS.



