# Native clipboard node module (Macos and Windows)

Native node addon that allows you to manage the clipboard. Mainly to be used with [Electron](https://github.com/electron/electron).

## Installation

This module is installed via npm:

```
npm install --save @valudio/native-clipboard
```

## Usage

```javascript
import nativeClipboard from '@valudio/native-clipboard'
```

## API

### nativeClipboard.getFromClipboard ()

Returns the current value on the clipboard.

```javascript
const clipboardValue = nativeClipboard.getFromClipboard()
```

### nativeClipboard.setToClipboard (value)

Sets a value on the clipboard.

```javascript
nativeClipboard.setToClipboard('this is a test')
console.log(nativeClipboard.getFromClipboard()) // this is a test
```

### nativeClipboard.setSelectionToClipboard ()

Set whatever is selected to the clipboard.

```javascript
nativeClipboard.setSelectionToClipboard()
```

## Supported OSes
 * Windows
 * macOS

## License
[MIT](LICENSE) © [Valudio](http://valudio.com)
