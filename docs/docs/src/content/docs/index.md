---
title: 技术支持助手
description: 主页
template: doc
head:
  - tag: style
    content: |
      .content-panel:has(h1) {
        display: none !important;
      }
---

<style>
  .right-sidebar-container {
    display: none !important;
  }
  .main-pane {
    width: 100% !important;
    max-width: 100% !important;
  }
  main {
    width: 100% !important;
    max-width: 100% !important;
  }
  .sl-container {
    max-width: 100% !important;
  }
  .sl-markdown-content {
    max-width: 100% !important;
  }
</style>

<script>
  // 禁止页面滚动
  document.body.style.overflow = 'hidden';
  document.documentElement.style.overflow = 'hidden';
  window.addEventListener('wheel', e => e.preventDefault(), { passive: false });
  window.addEventListener('touchmove', e => e.preventDefault(), { passive: false });
</script>

<div class="iframe-wrapper">
  <iframe
    src="https://se-support-mx-cinny.pages.dev/"
    frameborder="0"
    allowfullscreen>
  </iframe>
</div>

<style>
  .iframe-wrapper {
    width: 100%;
    height: calc(100vh - 160px);
  }

  .iframe-wrapper iframe {
    width: 100%;
    height: 100%;
    border: none;
  }
</style>
