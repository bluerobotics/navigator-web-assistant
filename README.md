# Navigator Assistant

Navigator Assistant is a webserver that interfaces with navigator-rs, the Rust navigator library.
With this application, users can run a server locally and control navigator from web interfaces (Websocket and RestAPI).

Navigator Assistant is available at BlueOS extensions, try it:
```
blueos.local/tools/extensions-manager
```
And add Navigator Assistant Extension. Or manually install from source from this [instructions].

You can also use the Jupyter extension to try some fast python and websocket applications.

[instructions]: https://github.com/RaulTrombin/blueos-navigator-assitant
## Instructions

To use navigator assistant simply:
```
./navigator-assistant
```

To enable datalogger service with default values:
```
./navigator-assistant --enable true
```

To change datalogger service values:
```
./navigator-assistant --enable true --rate 60000 --directory ./ --filename data.csv
```

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
Multiple commands can be sent, just use the '&&', try:
```
/output/userled/led1/true &&
/output/neopixel/100/100/0

&& /output/pwm/enable/false
```
The requests will work even with inner trailing spaces.
