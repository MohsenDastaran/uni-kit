<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref } from "vue";

/**
 * The ThreeUI "Matrix Field" WebGL, ported verbatim from the registered source
 * (matrix-field.json). It is the pointer-reactive three-way matrix junction:
 * three lightning beams radiating from a centre, a pulse travelling each beam,
 * and lightning that reaches toward the pointer while it moves.
 *
 * React and its runtime are gone; the vertex/fragment shaders and the pointer
 * ramp are copied as authored. The source's `patch()` folds `size` and `length`
 * into the beam intensity, which is reproduced below; the other knobs are
 * accepted but unused for this variant, exactly as in the source.
 *
 * `pointer` turns the reach toward the cursor off, leaving only the beams and
 * their travelling pulses. The hero runs with it off.
 */
const props = withDefaults(
  defineProps<{
    size?: number;
    length?: number;
    speed?: number;
    density?: number;
    opacity?: number;
    hue?: number;
    saturation?: number;
    brightness?: number;
    pointer?: boolean;
  }>(),
  {
    size: 1,
    length: 1,
    speed: 0.2,
    density: 1,
    opacity: 1,
    hue: 0,
    saturation: 1,
    brightness: 1,
    pointer: true,
  },
);

const canvas = ref<HTMLCanvasElement>();

const VERTEX = `
attribute vec4 aVertexPosition;
void main() {
  gl_Position = aVertexPosition;
}
`;

const FRAGMENT = `
precision highp float;
uniform vec2 u_resolution;
uniform float u_time;
uniform vec2 u_mouse;
uniform float u_mouseActive;

float hash(float n) { return fract(sin(n)*753.5453123); }
float noise(float x) {
    float i = floor(x);
    float f = fract(x);
    f = f*f*(3.0-2.0*f);
    return mix(hash(i), hash(i+1.0), f);
}

vec2 sdLine(vec2 p, vec2 a, vec2 b) {
    vec2 pa = p - a, ba = b - a;
    float h = clamp(dot(pa, ba) / dot(ba, ba), 0.0, 1.0);
    return vec2(length(pa - ba * h), h);
}

float lightning(vec2 uv, vec2 a, vec2 b, float t) {
    vec2 ab = b - a;
    float len = length(ab);
    if(len < 0.01) return 0.0;
    vec2 dir = ab / len;

    vec2 pa = uv - a;
    float h = clamp(dot(pa, dir) / len, 0.0, 1.0);
    float dist = length(pa - dir * (h * len));

    float env = sin(h * 3.1415);

    float offset = (noise(h * 25.0 - t * 35.0) - 0.5) * 0.08 * env;
    offset += (noise(h * 70.0 + t * 50.0) - 0.5) * 0.02 * env;

    float d = abs(dist + offset);

    return (0.0002 / (d + 0.0002) + 0.00001 / (d*d + 0.00001)) * env;
}

void main() {
    vec2 uv = gl_FragCoord.xy / u_resolution.xy;
    uv = uv * 2.0 - 1.0;
    uv.x *= u_resolution.x / u_resolution.y;

    vec2 mouseUV = u_mouse / u_resolution.xy;
    mouseUV = mouseUV * 2.0 - 1.0;
    mouseUV.x *= u_resolution.x / u_resolution.y;

    vec2 center = vec2(-0.8, -0.2);
    center.x += sin(u_time * 0.4) * 0.03;
    center.y += cos(u_time * 0.3) * 0.03;

    vec2 dirUp = normalize(vec2(0.15, 1.0));
    vec2 dirRight = normalize(vec2(1.0, -0.25));
    vec2 dirDownLeft = normalize(vec2(-0.8, -0.6));

    vec2 l1 = sdLine(uv, center, center + dirUp * 5.0);
    vec2 l2 = sdLine(uv, center, center + dirRight * 5.0);
    vec2 l3 = sdLine(uv, center, center + dirDownLeft * 5.0);

    float intensity = ${(0.006 * props.size * props.length).toFixed(5)};
    float glow = intensity / (l1.x + 0.001) +
                 intensity / (l2.x + 0.001) +
                 (intensity * 0.4) / (l3.x + 0.001);

    float pulse1 = smoothstep(0.1, 0.0, abs(l1.y - fract(u_time * 0.4))) * 0.03 / (l1.x + 0.001);
    float pulse2 = smoothstep(0.1, 0.0, abs(l2.y - fract(u_time * 0.5 + 0.3))) * 0.03 / (l2.x + 0.001);
    float pulse3 = smoothstep(0.1, 0.0, abs(l3.y - fract(u_time * 0.3 + 0.7))) * 0.015 / (l3.x + 0.001);
    glow += pulse1 + pulse2 + pulse3;

    vec2 p1 = center + dirUp * clamp(dot(mouseUV - center, dirUp), 0.0, 5.0);
    vec2 p2 = center + dirRight * clamp(dot(mouseUV - center, dirRight), 0.0, 5.0);
    vec2 p3 = center + dirDownLeft * clamp(dot(mouseUV - center, dirDownLeft), 0.0, 5.0);

    float lgt1 = lightning(uv, p1, mouseUV, u_time);
    float lgt2 = lightning(uv, p2, mouseUV, u_time + 10.0);
    float lgt3 = lightning(uv, p3, mouseUV, u_time + 20.0);

    float flicker = step(0.1, noise(u_time * 60.0)) * (noise(u_time * 150.0) * 0.8 + 0.2);

    float d1 = length(mouseUV - p1);
    float d2 = length(mouseUV - p2);
    float d3 = length(mouseUV - p3);

    glow += lgt1 * smoothstep(2.0, 0.0, d1) * u_mouseActive * flicker;
    glow += lgt2 * smoothstep(2.0, 0.0, d2) * u_mouseActive * flicker;
    glow += lgt3 * smoothstep(2.0, 0.0, d3) * u_mouseActive * flicker;

    float distToCenter = length(uv - center);
    glow += 0.04 / (distToCenter + 0.01);

    vec3 baseColor = vec3(0.6, 0.75, 1.0);
    vec3 finalColor = baseColor * glow;

    finalColor *= 0.85 + 0.15 * sin(u_time * 2.0 - distToCenter * 8.0);

    float vignette = 1.0 - smoothstep(0.4, 2.0, length(uv));
    finalColor *= vignette;

    float n = fract(sin(dot(gl_FragCoord.xy, vec2(12.9898, 78.233))) * 43758.5453);
    finalColor += n * 0.02;

    // Premultiplied, so only the beams are opaque: everywhere else the canvas
    // is a window onto the surface behind it, which is what lets the hero wear
    // the page's own theme instead of a black sheet.
    float alpha = clamp(max(max(finalColor.r, finalColor.g), finalColor.b), 0.0, 1.0);
    gl_FragColor = vec4(finalColor, alpha);
}
`;

let gl: WebGLRenderingContext | null = null;
let program: WebGLProgram | null = null;
let raf = 0;
let lastMouseMove = 0;
let currentMouseActive = 0;
let mouseX = 0;
let mouseY = 0;
let startedAt = 0;
let reducedMotion = false;

function compile(glc: WebGLRenderingContext, type: number, source: string) {
  const shader = glc.createShader(type);
  if (!shader) return null;
  glc.shaderSource(shader, source);
  glc.compileShader(shader);
  if (!glc.getShaderParameter(shader, glc.COMPILE_STATUS)) {
    glc.deleteShader(shader);
    return null;
  }
  return shader;
}

function resize() {
  const c = canvas.value;
  if (!c || !gl) return;
  if (c.width !== c.clientWidth || c.height !== c.clientHeight) {
    c.width = c.clientWidth;
    c.height = c.clientHeight;
  }
}

function draw(now: number) {
  const c = canvas.value;
  if (!c || !gl || !program) return;
  // Hidden, the canvas reports no size. Do nothing rather than draw into a
  // zero-sized viewport: the loop keeps its frame so it resumes when shown.
  if (!c.clientWidth || !c.clientHeight) return;
  resize();
  gl.viewport(0, 0, c.width, c.height);
  gl.useProgram(program);

  const timeSinceMove = now - lastMouseMove;
  const targetActive =
    timeSinceMove < 150 ? 1 : Math.max(0, 1 - (timeSinceMove - 150) / 350);
  currentMouseActive += (targetActive - currentMouseActive) * 0.15;

  const u = {
    resolution: gl.getUniformLocation(program, "u_resolution"),
    time: gl.getUniformLocation(program, "u_time"),
    mouse: gl.getUniformLocation(program, "u_mouse"),
    active: gl.getUniformLocation(program, "u_mouseActive"),
  };
  gl.uniform2f(u.resolution, c.width, c.height);
  gl.uniform1f(u.time, reducedMotion ? 0 : (now - startedAt) * 0.001);
  gl.uniform2f(u.mouse, mouseX, mouseY);
  gl.uniform1f(u.active, props.pointer && !reducedMotion ? currentMouseActive : 0);

  gl.drawArrays(gl.TRIANGLE_STRIP, 0, 4);
}

function loop(now: number) {
  draw(now);
  if (!reducedMotion) raf = requestAnimationFrame(loop);
}

function onMove(event: MouseEvent) {
  const c = canvas.value;
  if (!c) return;
  const rect = c.getBoundingClientRect();
  mouseX = event.clientX - rect.left;
  // gl_FragCoord counts up from the bottom, so the pointer has to as well.
  // Passing screen coordinates here inverted the junction's reach.
  mouseY = rect.height - (event.clientY - rect.top);
  lastMouseMove = performance.now();
  if (reducedMotion) {
    reducedMotion = false;
    startedAt = performance.now();
    raf = requestAnimationFrame(loop);
  }
}

onMounted(() => {
  const c = canvas.value;
  if (!c) return;
  const ctx = c.getContext("webgl");
  if (!ctx) return;
  gl = ctx;

  const vs = compile(gl, gl.VERTEX_SHADER, VERTEX);
  const fs = compile(gl, gl.FRAGMENT_SHADER, FRAGMENT);
  if (!vs || !fs) return;
  program = gl.createProgram();
  if (!program) return;
  gl.attachShader(program, vs);
  gl.attachShader(program, fs);
  gl.linkProgram(program);
  if (!gl.getProgramParameter(program, gl.LINK_STATUS)) return;

  const buffer = gl.createBuffer();
  gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
  gl.bufferData(
    gl.ARRAY_BUFFER,
    new Float32Array([1, 1, -1, 1, 1, -1, -1, -1]),
    gl.STATIC_DRAW,
  );
  gl.useProgram(program);
  const loc = gl.getAttribLocation(program, "aVertexPosition");
  gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
  gl.vertexAttribPointer(loc, 2, gl.FLOAT, false, 0, 0);
  gl.enableVertexAttribArray(loc);

  startedAt = performance.now();
  lastMouseMove = startedAt;
  reducedMotion = window.matchMedia("(prefers-reduced-motion: reduce)").matches;

  if (props.pointer) window.addEventListener("mousemove", onMove);
  draw(performance.now());
  if (!reducedMotion) raf = requestAnimationFrame(loop);
});

onBeforeUnmount(() => {
  cancelAnimationFrame(raf);
  window.removeEventListener("mousemove", onMove);
  if (gl) {
    gl.deleteProgram(program);
    gl = null;
    program = null;
  }
});
</script>

<template>
  <canvas
    ref="canvas"
    class="laser-field"
    :style="{ opacity: opacity, filter: `brightness(${brightness})` }"
    aria-hidden="true"
  />
</template>
