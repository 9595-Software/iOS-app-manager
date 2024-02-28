# iOS-app-manager

ios-app-manager is a cli tool used for installing and starting iOS applications on a physical device without having to know the Device ID or the ever-changing installation URL. All that is required is the device name and the path to the app.

```
$ ./ios-app-manager --help                                                                                                 2 ↵
Install and launch iOS applications on a physical device

Usage: ios-app-manager [OPTIONS] --device-name <DEVICE_NAME> --app-path <APP_PATH>

Options:
  -d, --device-name <DEVICE_NAME>  
  -a, --app-path <APP_PATH>        
  -l, --launch                     
  -h, --help
```