export interface INativeClipboard {
  getFromClipboard: () => string
  setToClipboard: (value: string) => void
  setSelectionToClipboard: () => void
}

const addon: INativeClipboard = require('./index.node')
export default addon
