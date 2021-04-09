# Your First Bluetooth Low Energy App with Flutter

![Flutter App with Bluetooth Low Energy running on a real Android phone, connected to VSCode Debugger](https://lupyuen.github.io/images/flutter-title.png)

_Flutter App with Bluetooth Low Energy running on a real Android phone, connected to VSCode Debugger_

📝 _4 Jun 2020_

Ready to create your very first _"Hello World"_ app with [__Flutter__](https://flutter.dev/)?

Why not make a sophisticated app that says...

_"Hello Bluetooth Low Energy gadgets nearby... Tell me what's inside you!"_

With Flutter, Bluetooth LE (Low Energy) apps for Android AND iOS are ridiculously easy to build, let me show you how!

# Download Flutter SDK

The Flutter SDK works on Windows, macOS and Linux (Intel, not Arm, [so Raspberry Pi is no-go](https://twitter.com/MisterTechBlog/status/1267755390814453760?s=20)).

1. [Download the Flutter SDK](https://flutter.dev/docs/get-started/install)

1. Unzip the Flutter SDK to our Home Directory. 

1. Add `flutter/bin` to our PATH.

    For macOS and Linux, we may edit `~/.bashrc` (or equivalent) and add this...

    ```bash
    export PATH="$PATH:$HOME/flutter/bin"
    ```

1. Open a new Command Prompt. Check the Flutter SDK by entering...

    ```bash
    flutter
    ```

    We should see this helpful message...

![Flutter Tool](https://lupyuen.github.io/images/flutter-doctor1.png)

# Install Flutter Tools

1. [Download and install VSCode](https://code.visualstudio.com/)

1. At the Command Prompt, enter...

    ```bash
    flutter doctor
    ```

    We will see something like this...

    ![Flutter Doctor](https://lupyuen.github.io/images/flutter-doctor2.png)

1. Whoa that's a long list of complaints! But we shall fix only 3 things: __Android Toolchain__ (or __Xcode for iOS__), __VSCode__ and __Connected Device__

Let's fix them now...

# Flutter for Android

_(If you're building for iPhone, skip to the next section)_

1. __Android Toolchain__: Follow the instructions shown in your screen. 
    
    You may need to run `sdkmanager` and `flutter doctor --android-licenses`

1. __VSCode__: Launch VSCode. Click `View → Extensions`
    
    Install the Flutter Extension for VSCode...

    ![Flutter Extension for VSCode](https://lupyuen.github.io/images/flutter-vscode.png)

1. __Connected Device__: Connect our Android phone (with debugging enabled) to the USB port...

    ![Connect phone to USB port](https://lupyuen.github.io/images/flutter-usb.jpg)

1. After connecting our Android phone, we should see the phone in the VSCode status bar...

    ![Flutter Device in VSCode](https://lupyuen.github.io/images/flutter-device.png)

1. At the Command Prompt, enter...

    ```bash
    flutter -v devices
    ```

    We should see our phone...

    ```text
    List of devices attached
    99031FFG device usb:3376X product:coral model:Pixel_4_XL device:coral
    Pixel 4 XL • 99031FFG • android-arm64 • Android 10 (API 29)
    ```

1. Finally enter...

    ```bash
    flutter doctor
    ```

    We should see ticks for __Flutter__, __Android Toolchain__, __VSCode__ and __Connected Device__...

    ![Flutter Doctor After Fixes](https://lupyuen.github.io/images/flutter-doctor3.png)

1. We may ignore the other issues for now

# Flutter for iOS

_(If you're building for Android, skip to the next section)_

Let's fix 3 things shown below: __Xcode__, __VSCode__ and __Connected Device__

![Flutter Doctor](https://lupyuen.github.io/images/flutter-doctor2.png)

1. __Xcode__: Follow the instructions shown in your screen. 

    You may need to install Xcode, CocoaPods and run `xcodebuild`
    
1. __VSCode__: Launch VSCode. Click `View → Extensions`
    
    Install the Flutter Extension for VSCode...

    ![Flutter Extension for VSCode](https://lupyuen.github.io/images/flutter-vscode.png)

1. __Connected Device__: Connect our iPhone to the USB port.

1. After connecting our iPhone, we should see the phone in the VSCode status bar...

    ![Flutter on iOS](https://lupyuen.github.io/images/flutter-deviceios.png)

1. At the Command Prompt, enter...

    ```bash
    flutter -v devices
    ```

    We should see our phone...

    ```text
    1 connected device:
    iPhone 6 Plus • ios • iOS 12.4.6
    ```

1. Finally enter...

    ```bash
    flutter doctor
    ```

    We should see ticks for __Flutter__, __Xcode__, __VSCode__ and __Connected Device__...

    ![Flutter Doctor After Fixes](https://lupyuen.github.io/images/flutter-doctorios.png)

1. We may ignore the other issues for now

# Download Source Code for Flutter App

The source code for our Flutter app is located here...

[`github.com/lupyuen/flutter-blue-sample`](https://github.com/lupyuen/flutter-blue-sample)

Our app is derived from the [sample app](https://github.com/pauldemarco/flutter_blue/tree/master/example) that comes with the [`flutter_blue`](https://github.com/pauldemarco/flutter_blue) Bluetooth LE plugin for Flutter.

1. In VSCode, click `View → Command Palette`

1. Enter `Git Clone`

1. Enter `https://github.com/lupyuen/flutter-blue-sample`

1. Select a folder to download the source code

1. When prompted to open the cloned repository, click `Open`

1. When prompted to get missing packages, click `Get Packages`

Check this video for the steps to download the source code for our Flutter app...

- [Watch on YouTube](https://youtu.be/QSrg9DgLwjk)

- [Download the video](https://github.com/lupyuen/pinetime-rust-mynewt/releases/download/v4.2.1/flutter-debug.mov)

# Debug Flutter App

We're now ready to debug our Flutter app on a real Android or iOS phone!

1. In VSCode, click `Run → Start Debugging`

    ![Start Debugging in VSCode](https://lupyuen.github.io/images/flutter-debug1.png)

1. Select the `Dart & Flutter` debugger

    ![Select Debugger in VSCode](https://lupyuen.github.io/images/flutter-debug2.png)

1. Wait for the Flutter app to be compiled and deployed to our phone (May take a minute for the first time)

1. For iOS: Check the next section for additional Xcode steps

1. When the Flutter app starts, we'll be able to Scan, Connect, Reload and Expand devices over Bluetooth LE like this...

![Scanning for Bluetooth LE devices](https://lupyuen.github.io/images/flutter-scan.png)

And that's our very first Flutter app with Bluetooth LE!

VSCode Debugger has many useful features for debugging Flutter apps. Here's what we see when we hit an unhandled exception in our Flutter app...

![Flutter App with VSCode Debugger](https://lupyuen.github.io/images/flutter-debug.png)

[_Larger image_](https://lupyuen.github.io/images/flutter-debug.png)

- __Dev Tools__: Shows the widgets rendered in our app

- __Variables__: Shows the local and global variables and their values

- __Call Stack__: Function calls leading to the exception or breakpoint

- __Debug Console__: Compilation, deployment and runtime messages

- __Source Code__: Shows the line of code for the exception or breakpoint

- __Debug Toolbar__: Resume execution, step into functions, step over code, hot reload, restart execution... [More about debugging](https://code.visualstudio.com/docs/editor/debugging)
    
See this article for more details on building Flutter apps with VSCode, including cool features like Hot Reload...

- [Flutter Development on VSCode](https://flutter.dev/docs/development/tools/vs-code)

Here's a video of the steps for debugging a Flutter app with VSCode...

- [Watch on YouTube](https://youtu.be/QSrg9DgLwjk)

- [Download the video](https://github.com/lupyuen/pinetime-rust-mynewt/releases/download/v4.2.1/flutter-debug.mov)

# Sign Flutter App for iOS

_(Skip this section if you're building for Android)_

This message appears when we debug our iOS app...

![Xcode Signing](https://lupyuen.github.io/images/flutter-sign.png)

Here's what we need to do for iOS...

1. In VSCode, click `Terminal → New Terminal`

1. At the Terminal prompt, enter...

    ```bash
    open ios/Runner.xcworkspace
    ```

    ![Open Xcode workspace](https://lupyuen.github.io/images/flutter-workspace.png)

1. In Xcode, click `Runner → Targets Runner → Signing & Capabilities`

    ![Xcode Signing](https://lupyuen.github.io/images/flutter-sign2.png)

1. Set `Team` to our Apple Developer Account

1. Set `Bundle Identifier` to a unique name

1. On our iPhone, click `Settings → General → Device Management`

1. Set the Trust Settings like this...

    ![Trust iOS Developer](https://lupyuen.github.io/images/flutter-trust.png)

We should be able to launch and debug our Flutter app using the instructions from the previous section...

![Flutter App on iOS](https://lupyuen.github.io/images/flutter-ios.jpg)

Here's a demo of our Flutter app on iPhone...

- [Watch on YouTube](https://youtu.be/MTBEd8xRrpA)

- [Download the video](https://github.com/lupyuen/pinetime-rust-mynewt/releases/download/v4.2.1/flutter-ios.mov)


![PineTime Smart Watch](https://lupyuen.github.io/images/micropython-title.jpg)

_PineTime Smart Watch_

# Bluetooth LE Services

Let's connect our Flutter app to a [__PineTime Smart Watch__](https://wiki.pine64.org/index.php/PineTime)...

- [Watch on YouTube](https://youtu.be/pt-BYs_7qOE)

- [Download the video](https://github.com/lupyuen/pinetime-rust-mynewt/releases/download/v4.2.1/flutter-pinetime-rotated.mp4)

_So many Services and Characteristics... What are they?_

When we access data and perform functions wirelessly on a Bluetooth LE device (like PineTime), we talk via a Bluetooth LE protocol known as the __Generic Attribute (GATT) Profile__. 

GATT defines the standard way for a Bluetooth LE Client (like our Flutter app) to access a Bluetooth LE Service (like on the PineTime Smart Watch).  [More about GATT](https://learn.adafruit.com/introduction-to-bluetooth-low-energy/gatt)

Our Flutter app displays the GATT Services and GATT Characteristics supported by the Bluetooth LE device. Let's look at the [__Standard GATT Services__](https://www.bluetooth.com/specifications/gatt/services/) that are defined in the Bluetooth LE Specifications...

![Bluetooth LE Services on PineTime Smart Watch](https://lupyuen.github.io/images/flutter-services.png)

1. __Generic Access__ (`0x1800`):
This GATT Service contains two GATT Charactertistics: Device Name (`pinetime`) and Appearance. [Specifications](https://www.bluetooth.com/xml-viewer/?src=https://www.bluetooth.com/wp-content/uploads/Sitecore-Media-Library/Gatt/Xml/Services/org.bluetooth.service.generic_access.xml)

1. __Generic Attribute__ (`0x1801`): This GATT Services notifies our Flutter app of any changes in PineTime's GATT Services.
[Specifications](https://www.bluetooth.com/xml-viewer/?src=https://www.bluetooth.com/wp-content/uploads/Sitecore-Media-Library/Gatt/Xml/Services/org.bluetooth.service.generic_attribute.xml)

1. __Device Information__ (`0x180A`): Contains two GATT Charactertistics: Model Number (`Apache Mynewt NimBLE`) and Firmware Revision (`1.0.0`).
[Specifications](https://www.bluetooth.com/xml-viewer/?src=https://www.bluetooth.com/wp-content/uploads/Sitecore-Media-Library/Gatt/Xml/Services/org.bluetooth.service.device_information.xml)

1. __Alert Notification Service__ (`0x1811`): For Alerts and Notifications.
[Specifications](https://www.bluetooth.com/xml-viewer/?src=https://www.bluetooth.com/wp-content/uploads/Sitecore-Media-Library/Gatt/Xml/Services/org.bluetooth.service.alert_notification.xml)

Some GATT Characteristics are shown as a list of numbers...

```text
    Device Name:
    Characteristic 0x2A00
    [112, 105, 110, ...]
```

The numbers are the ASCII codes for the text strings. We can see the actual strings in the Nordic nRF Connect app below.

_Can we do complex things with GATT? Like update the firmware wirelessly on PineTime?_

There's a custom GATT Service and Characteristic for that!

__Simple Management Protocol__ (`8D53DC1D-1DB7-4CD3-868B-8A527460AA84`, shortened to `0xDC1D` above) is the GATT Service used by PineTime for updating the firmware.

The GATT Service contains a single GATT Characteristic (`DA2E7828-FBCE-4E01-AE9E-261174997C48`, shortened to `0x7828` above). Our Flutter app may someday update PineTime's firmware by sending a Write Request to this GATT Characteristic (with the firmware file encoded in the request).

[More about Wireless Firmware Update on PineTime](https://lupyuen.github.io/pinetime-rust-mynewt/articles/dfu)

The final GATT Service (`59462f12-9543-9999-12c8-58b459a2712d`, shorted to `0x2F12` above) is the __Security Test Service__. [More details](https://github.com/apache/mynewt-nimble/blob/master/apps/btshell/src/gatt_svr.c#L67-L94)

For comparison, here are the GATT Services that appear when the [Nordic nRF Connect](https://www.nordicsemi.com/Software-and-tools/Development-Tools/nRF-Connect-for-mobile) mobile app is connected to PineTime. So our Flutter app really works!

![Nordic nRF Connect app connected to PineTime](https://lupyuen.github.io/images/dfu-gattservices.jpg)

# Bluetooth LE Code

I'm new to Flutter and Dart... And I find it absolutely amazing that a few lines of code can do so much!

Our app is structured like this to scan Bluetooth LE devices and display them...

![Our App Structure](https://lupyuen.github.io/images/flutter-structure.png)

Here's the code that implements the screen for scanning Bluetooth LE devices: [`lib/main.dart`](https://github.com/lupyuen/flutter-blue-sample/blob/master/lib/main.dart#L67-L153)

```dart
//  Screen for finding devices
class FindDevicesScreen extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      //  Title for the screen
      appBar: AppBar(
        title: Text('Find Devices'),
      ),

      body: RefreshIndicator(
        //  Start scanning for Bluetooth LE devices
        onRefresh: () =>
            FlutterBlue.instance.startScan(timeout: Duration(seconds: 4)),

        //  List of Bluetooth LE devices
        child: SingleChildScrollView(
          child: Column(
            children: <Widget>[
              ...
              StreamBuilder<List<ScanResult>>(
                stream: FlutterBlue.instance.scanResults,
                initialData: [],

                builder: (c, snapshot) => Column(
                  children: snapshot.data
                      .map(
                        //  For each Bluetooth LE device, show the ScanResultTile widget when tapped
                        (r) => ScanResultTile(
                          result: r,
                          onTap: () => Navigator.of(context)
                              .push(MaterialPageRoute(builder: (context) {
                            r.device.connect();
                            return DeviceScreen(device: r.device);
                          })),
                        ),
                      )
                      .toList(),
                ),
              ),
            ...
```

And here's the code that renders each Bluetooth LE device found: [`lib/widgets.dart`](https://github.com/lupyuen/flutter-blue-sample/blob/master/lib/widgets.dart#L8-L121)

```dart
//  Widget for displaying a Bluetooth LE device
class ScanResultTile extends StatelessWidget {
  ...
  @override
  Widget build(BuildContext context) {
    return ExpansionTile(
      //  Show the device name and signal strength
      title: _buildTitle(context),
      leading: Text(result.rssi.toString()),

      //  Show the Connect button and call onTap when tapped
      trailing: RaisedButton(
        child: Text('CONNECT'),
        color: Colors.black,
        textColor: Colors.white,
        onPressed: (result.advertisementData.connectable) ? onTap : null,
      ),

      //  Display the device's name, power level, manufacturer data, service UUIDs and service data
      children: <Widget>[
        _buildAdvRow(
            context, 'Complete Local Name', result.advertisementData.localName),
        _buildAdvRow(context, 'Tx Power Level',
            '${result.advertisementData.txPowerLevel ?? 'N/A'}'),
        _buildAdvRow(
            context,
            'Manufacturer Data',
            getNiceManufacturerData(
                result.advertisementData.manufacturerData) ??
                'N/A'),
        _buildAdvRow(
            context,
            'Service UUIDs',
            (result.advertisementData.serviceUuids.isNotEmpty)
                ? result.advertisementData.serviceUuids.join(', ').toUpperCase()
                : 'N/A'),
        _buildAdvRow(context, 'Service Data',
            getNiceServiceData(result.advertisementData.serviceData) ?? 'N/A'),
      ],
    );
  }
  ...
```

Yes the code looks similar to JavaScript. (Because Dart was designed to [compile to JavaScript efficiently](https://dart.dev/faq#q-why-isnt-dart-more-like-haskell--smalltalk--python--scala--other-language))

But overall the user interface code looks Declarative and Functional... A huge improvement over JavaScript and React Native!

_How do we call the Bluetooth LE functions in our own Flutter app?_

Just add the [`flutter_blue`](https://github.com/pauldemarco/flutter_blue) plugin as a dependency like this: [`pubspec.yaml`](https://github.com/lupyuen/flutter-blue-sample/blob/master/pubspec.yaml)

```text
dependencies:
  flutter_blue: ^0.7.2
```

# What's Next

I'll be using the code in this article to create the open source __PineTime Companion App__ for Android and iOS. So that we can flash our PineTime Smart Watches wirelessly, sync the date and time, show notifications, chart our heart rate, ...

Flutter makes it really easy to maintain a single code base for Android and iOS... And `flutter_blue` makes Bluetooth LE coding so simple!

If you're keen to help out, come chat with the PineTime FOSS Community (and me) in the PineTime Chatroom!

[PineTime Chatroom on Matrix / Discord / Telegram / IRC](https://wiki.pine64.org/index.php/PineTime#Community)

-   [Sponsor me a coffee](https://github.com/sponsors/lupyuen)

-   [Discuss this article on Pine64 Forum](https://forum.pine64.org/showthread.php?tid=10050)

-   [Check out my articles](https://lupyuen.github.io)

-   [RSS Feed](https://lupyuen.github.io/rss.xml)

_Got a question, comment or suggestion? Create an Issue or submit a Pull Request here..._

[`pinetime-rust-mynewt/rust/ app/src/flutter.md`](https://github.com/lupyuen/pinetime-rust-mynewt/blob/master/rust/app/src/flutter.md)

# Further Reading

_["MCUBoot Bootloader for PineTime Smart Watch (nRF52)"](https://lupyuen.github.io/pinetime-rust-mynewt/articles/mcuboot)_

_["Firmware Update over Bluetooth Low Energy on PineTime Smart Watch"](https://lupyuen.github.io/pinetime-rust-mynewt/articles/dfu)_

_["Wireless Firmware Update In Action on PineTime Smart Watch (nRF52)"](https://lupyuen.github.io/pinetime-rust-mynewt/articles/dfutest)_
