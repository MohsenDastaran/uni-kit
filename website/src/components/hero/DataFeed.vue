<script setup lang="ts">
/**
 * The reverseui "DataFeedingIn", ported verbatim from its source. The seven
 * dashed paths, the gradient mask, and the seven travelling pulse gradients are
 * copied as authored; framer-motion's `x1`/`x2` animation is the SVG SMIL
 * `<animate>` below with the same 1.5s linear loop, 0.25s delay, and the rows'
 * staggered fade-in is CSS with the same delays.
 */
const PATHS = [
  "M0 100H55.022C61.8914 100 68.6451 101.769 74.6324 105.137L120.368 130.863C126.355 134.231 133.109 136 139.978 136H201.5",
  "M0 60H48.2171C59.2463 60 69.7861 64.5539 77.3451 72.5854L117.655 115.415C125.214 123.446 135.754 128 146.783 128H201.5",
  "M0 188H55.022C61.8914 188 68.6451 186.231 74.6324 182.863L120.368 157.137C126.355 153.769 133.109 152 139.978 152H201.5",
  "M0 228H48.2171C59.2463 228 69.7861 223.446 77.3451 215.415L117.655 172.585C125.214 164.554 135.754 160 146.783 160H201.5",
  "M0 287H41.7852C56.4929 287 70.0142 278.929 76.994 265.983L118.49 189.017C125.47 176.071 138.991 168 153.699 168H202",
  "M0 144L201 145",
  "M0 1H41.5946C56.3171 1 69.8495 9.08744 76.823 22.0537L118.177 98.9463C125.15 111.913 138.683 120 153.405 120H201.5",
];
</script>

<template>
  <div class="data-feed">
    <div class="data-feed__pulse">
      <svg viewBox="0 0 202 288" fill="none">
        <template v-for="(d, index) in PATHS" :key="index">
          <path
            :d="d"
            stroke="currentColor"
            mask="url(#df-mask)"
            stroke-linecap="round"
            stroke-opacity="0.2"
            stroke-width="2"
            stroke-dasharray="0.1 3"
          />
          <path
            :d="d"
            :stroke="`url(#df-pulse-${index})`"
            stroke-linecap="round"
            stroke-opacity="1"
            stroke-width="2"
            stroke-dasharray="0.1 3"
            mask="url(#df-mask)"
          />
        </template>
        <defs>
          <linearGradient id="df-mask-grad" x1="202" y1="227" x2="32" y2="227" gradientUnits="userSpaceOnUse">
            <stop stop-color="currentColor" />
            <stop offset="1" stop-color="currentColor" stop-opacity="0" />
          </linearGradient>
          <mask id="df-mask" maskUnits="userSpaceOnUse">
            <rect width="202" height="288" fill="url(#df-mask-grad)" />
          </mask>
          <template v-for="index in 7" :key="`g${index}`">
            <linearGradient
              :id="`df-pulse-${index - 1}`"
              x1="-100%"
              y1="0"
              x2="0"
              y2="0"
              gradientUnits="userSpaceOnUse"
            >
              <stop offset="0.35" stop-color="#716FFF" stop-opacity="0" />
              <stop offset="0.45" stop-color="#716FFF" />
              <stop offset="0.55" stop-color="#716FFF" />
              <stop offset="0.65" stop-color="#716FFF" stop-opacity="0" />
              <animate attributeName="x1" values="-50%;100%" dur="1.5s" begin="0.25s" repeatCount="indefinite" calcMode="linear" />
              <animate attributeName="x2" values="50%;150%" dur="1.5s" begin="0.25s" repeatCount="indefinite" calcMode="linear" />
            </linearGradient>
          </template>
        </defs>
      </svg>
    </div>

    <div class="data-feed__card">
      <div class="data-feed__bar" aria-hidden="true">
        <i /><i /><i />
      </div>
      <div class="data-feed__head" aria-hidden="true">
        <div class="data-feed__cell" v-for="i in 3" :key="`h${i}`">
          <span class="data-feed__check" />
          <span class="data-feed__barline" />
        </div>
      </div>
      <div class="data-feed__body" aria-hidden="true">
        <div class="data-feed__row" v-for="i in 6" :key="`r${i}`" :style="{ animationDelay: `${1.25 + (i - 1) * 1.5}s` }">
          <div class="data-feed__cell" v-for="j in 3" :key="`c${j}`">
            <span class="data-feed__barline" />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.data-feed {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}

.data-feed__pulse {
  height: 5rem;
  transform-origin: right center;
  transform: rotate(90deg) translateX(2.5rem) translateY(0);
  color: #716fff;
}

.data-feed__pulse svg {
  width: 202px;
  height: auto;
  margin-left: auto;
  display: block;
}

.data-feed__card {
  width: 100%;
  max-width: 400px;
  height: 260px;
  overflow: hidden;
  border-radius: 14px;
  background: linear-gradient(
    color-mix(in srgb, var(--foreground) 5%, transparent) 0%,
    transparent 100%
  );
  border: 1px solid var(--border);
}

.data-feed__bar {
  display: flex;
  gap: 0.5rem;
  padding: 0.625rem;
  border-bottom: 1px solid var(--border);
}

.data-feed__bar i {
  width: 0.5rem;
  height: 0.5rem;
  border-radius: 999px;
  background: color-mix(in srgb, var(--foreground) 10%, transparent);
}

.data-feed__head,
.data-feed__row {
  display: flex;
  width: 100%;
  overflow: hidden;
}

.data-feed__head .data-feed__cell {
  height: 2.5rem;
  border-bottom: 1px solid var(--border);
  border-right: 1px solid var(--border);
}

.data-feed__cell {
  display: flex;
  flex-shrink: 0;
  align-items: center;
  gap: 0.5rem;
  width: 180px;
  padding: 0 0.5rem;
}

.data-feed__row .data-feed__cell {
  height: 2.25rem;
  border-bottom: 1px solid var(--border);
  border-right: 1px solid var(--border);
}

.data-feed__check {
  width: 1rem;
  height: 1rem;
  border-radius: 0.25rem;
  border: 2px solid color-mix(in srgb, var(--foreground) 8%, transparent);
  background: color-mix(in srgb, var(--foreground) 15%, transparent);
}

.data-feed__barline {
  display: block;
  width: 88px;
  height: 0.5rem;
  border-radius: 0.25rem;
  background: color-mix(in srgb, var(--foreground) 8%, transparent);
}

.data-feed__row {
  animation: data-feed-in 0.3s ease-in both;
}

@keyframes data-feed-in {
  from {
    transform: translateY(20px);
    opacity: 0;
  }
  to {
    transform: translateY(0);
    opacity: 1;
  }
}

@media (prefers-reduced-motion: reduce) {
  .data-feed__row {
    animation: none;
  }
  .data-feed__pulse svg animate {
    animation: none;
  }
}
</style>
