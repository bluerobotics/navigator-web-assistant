# Navigator WebAssistant

Navigator WebAssistant is a webserver that interfaces with navigator-rs, the Rust navigator library.
With this application, users can run a server locally and control navigator from web interfaces (Websocket and RestAPI).

Navigator WebAssistant running on BlueOS:
![Banner](page/navigator-webassistant-demo.gif)

Navigator WebAssistant is available at BlueOS extensions, try it:
```
blueos.local/tools/extensions-manager
```
And add Navigator WebAssistant Extension. Or manually install from source from this [instructions].

You can also use the Jupyter extension to try some fast python and websocket applications.

[instructions]: https://github.com/RaulTrombin/blueos-navigator-assitant
## Instructions

To use Navigator WebAssistant simply execute:
```
./navigator-webassistant
```

To disable navigator monitor:

```
./navigator-webassistant --monitor-rate 0
```

To enable datalogger service with default values:
```
./navigator-webassistant --datalogger-rate 1
```

To change datalogger service values:
```
./navigator-webassistant --datalogger-rate 0.0017 --datalogger-directory ./ --datalogger-filename data.csv
```

To change navigator monitor's refresh rate:

```
./navigator-webassistant --monitor-rate 10
```

*The rate values use Hz.

## Webpage

To get some access to navigator from web, you can start using the web hosted interface:
```
localhost/8080
```

## RestAPI

To get instructions on how to use API routes, access the docs:
```
localhost/8080/docs
```

## Websocket

To get access to navigator from websocket:
```
localhost/8080/ws
```
An optional regex filter can be applied on the register process, try:
```
localhost/8080/ws?filter=output
```
Users can send commands like the restAPI routes, try:
```
/input/all
```
For POST methods define the structures after, as follows:
```
/output/pwm/channel/value/{ "channel": "Ch9", "value": 10 }
```
Multiple commands can be sent, just use the '&&', try:
```
/output/pwm/enable/{ "enable": false } &&
/output/pwm/frequency/{ "frequency": 60 } &&

/output/pwm/channel/value/{ "channel": "Ch9", "value": 1000 } &&
/output/pwm/channel/value/{ "channel": "Ch10", "value": 3000 } &&
/output/pwm/enable/{ "enable": true }
```
The requests will work even with inner trailing spaces.
