/**
 * Starts work that would hold the main thread, but only once the reader is
 * still.
 *
 * A preview is a same-origin iframe, so it shares this page's thread: building
 * its WebAssembly module and drawing its first frame blocks long enough to
 * swallow the click that was about to navigate. On a component page that click
 * is the sidebar link to the next component, and the page reads as frozen until
 * the preview finishes starting.
 *
 * So the work waits for a quiet moment, and any click, key or scroll postpones
 * it. The actions that must stay immediate — moving between pages — win, and the
 * preview is there by the time the reader looks for it. Only one piece of work
 * runs per quiet window, since starting a preview is itself a gesture's worth of
 * blocking.
 */
const QUIET_MS = 1200;

/** What a reader does when they are not simply reading. */
const GESTURES = ['pointerdown', 'keydown', 'wheel', 'touchstart', 'scroll'];

let lastGesture = Date.now();
let watching = false;
let timer: number | undefined;
let queue: Array<() => void> = [];

function markGesture() {
  lastGesture = Date.now();
}

function watch() {
  if (watching) return;
  watching = true;
  for (const type of GESTURES) {
    window.addEventListener(type, markGesture, { passive: true, capture: true });
  }
}

function pump() {
  if (queue.length === 0) {
    window.clearInterval(timer);
    timer = undefined;
    return;
  }
  if (Date.now() - lastGesture < QUIET_MS) return;
  const [start, ...rest] = queue;
  queue = rest;
  start();
  // Starting the work blocks this thread, so give the reader the quiet window
  // again before the next piece.
  markGesture();
}

/** Runs `start` at the next quiet moment, and never before. */
export function whenQuiet(start: () => void) {
  // Nothing is rendering on the server, so there is no thread to protect and no
  // reader to wait for.
  if (typeof window === "undefined") {
    start();
    return;
  }
  watch();
  queue = [...queue, start];
  if (timer === undefined) timer = window.setInterval(pump, 250);
}

/** Drops everything that was waiting; nothing already started is affected. */
export function cancelQuiet() {
  queue = [];
  if (timer !== undefined) {
    window.clearInterval(timer);
    timer = undefined;
  }
}
