// Asking the embedding page to show a sample.
//
// NSH has no DOM and no `window.parent`: the app runs in QuickJS. The player
// page installs an `onLog` handler, so the demo writes one line to the console
// and the player turns it into the `postMessage` the component page accepts.
// Natively the line is just a log and the click does nothing.

const PREFIX = 'nsh-gallery-source:';

export function showSource(index: number, title: string): void {
  console.log(`${PREFIX}${JSON.stringify({ index, title })}`);
}
