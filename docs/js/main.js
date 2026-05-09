// ===== i18n =====
const i18n = {
  en: {
    // Nav
    nav_features: 'Features',
    nav_screenshots: 'Screenshots',
    nav_quickstart: 'Quick Start',
    // Hero
    hero_badge: 'Terminal-native · Rust-powered · Blazing fast',
    hero_title: 'Your SSH manager,<br>right in the terminal',
    hero_desc: 'A fully TUI-based SSH connection manager built with Rust. No browser, no Electron — just your terminal, butter-smooth and instant. Manage servers, encrypt credentials, and connect with a single keypress.',
    hero_install: 'Install Now',
    hero_source: 'View Source',
    // Features
    features_title: 'Everything you need, nothing you don\'t',
    features_desc: 'Powerful features designed for developers who live in the terminal',
    f_vault_title: 'Encrypted Vault',
    f_vault_desc: 'AES-256-GCM encryption with Argon2id key derivation keeps your credentials safe locally.',
    f_group_title: 'Group Organization',
    f_group_desc: 'Organize hosts into collapsible groups. Drag to reorder with Shift+J/K.',
    f_search_title: 'Instant Search',
    f_search_desc: 'Full-text search across host names, addresses, and groups. Find anything in milliseconds.',
    f_theme_title: '4 Built-in Themes',
    f_theme_desc: 'Catppuccin Mocha, Tokyo Night, Dracula, Gruvbox Dark — with live preview on switch.',
    f_i18n_title: 'Multilingual',
    f_i18n_desc: 'In-app language switching for English, Chinese, Japanese, and Korean.',
    f_vim_title: 'Vim Keybindings',
    f_vim_desc: 'Navigate with j/k/h/l. Every action mapped to a single keypress. No mouse needed.',
    // Screenshots
    screenshots_title: 'See it in action',
    screenshots_desc: 'A stunning TUI that makes managing SSH connections a joy',
    // Quick Start
    qs_title: 'Up and running in 30 seconds',
    qs_desc: 'Clone, build, and connect to your first server',
    step1_title: 'Install a Nerd Font',
    step1_desc: 'Required for icons. E.g. <code>brew install --cask font-jetbrains-mono-nerd-font</code>',
    step2_title: 'Clone &amp; Build',
    step2_desc: 'Requires Rust 1.85+. Build with <code>cargo build --release</code>',
    step3_title: 'Run Wormhole',
    step3_desc: 'Execute <code>wormhole</code> in your terminal. Set a master password or skip it.',
    // Code block
    copy: 'Copy',
    copied: 'Copied!',
    code_clone_comment: '# Clone the repository',
    code_build_comment: '# Build from source',
    code_install_comment: '# Install to PATH',
    code_launch_comment: '# Launch!',
    // Keybindings
    kb_title: 'Keyboard Shortcuts',
    kb_key: 'Key',
    kb_action: 'Action',
    kb_down: 'Move down',
    kb_up: 'Move up',
    kb_panel: 'Switch panel',
    kb_connect: 'Connect to host',
    kb_newhost: 'New host',
    kb_newgroup: 'New group',
    kb_edit: 'Edit selected',
    kb_delete: 'Delete selected',
    kb_search: 'Search',
    kb_theme: 'Switch theme',
    kb_help: 'Help',
    kb_quit: 'Quit',
    // Footer
    // CTA
    cta_title: 'Manage SSH — without ever leaving the terminal',
    cta_desc: 'Groups, encrypted credentials, instant search, beautiful themes — all in a single TUI. Built with Rust for speed. One command to install, one keypress to connect.',
    cta_install: 'Get Started',
    cta_star: 'Star on GitHub',
    footer_copy: '© 2025 Wormhole Contributors. Released under the MIT License.',
    footer_license: 'License',
  },
  zh: {
    nav_features: '功能特性',
    nav_screenshots: '截图',
    nav_quickstart: '快速开始',
    hero_badge: '终端原生 · Rust 驱动 · 极速渲染',
    hero_title: '你的 SSH 管理器，<br>就在终端里',
    hero_desc: '完全基于 TUI 的 SSH 连接管理器，使用 Rust 构建。无需浏览器、无需 Electron —— 只有你的终端，丝滑流畅、即时响应。管理服务器、加密凭证，一键连接。',
    hero_install: '立即安装',
    hero_source: '查看源码',
    features_title: '你所需要的一切，恰到好处',
    features_desc: '为终端开发者设计的强大功能',
    f_vault_title: '加密保险库',
    f_vault_desc: '使用 AES-256-GCM 加密与 Argon2id 密钥派生，确保凭证安全存储在本地。',
    f_group_title: '分组管理',
    f_group_desc: '将主机组织到可折叠的分组中，使用 Shift+J/K 重新排列。',
    f_search_title: '即时搜索',
    f_search_desc: '跨主机名、地址和分组的全文搜索，毫秒级响应。',
    f_theme_title: '4 种内置主题',
    f_theme_desc: 'Catppuccin Mocha、Tokyo Night、Dracula、Gruvbox Dark — 支持实时预览切换。',
    f_i18n_title: '多语言支持',
    f_i18n_desc: '应用内切换英语、中文、日语和韩语界面。',
    f_vim_title: 'Vim 快捷键',
    f_vim_desc: '使用 j/k/h/l 导航，每个操作映射到一个按键，无需鼠标。',
    screenshots_title: '实际效果',
    screenshots_desc: '精美的 TUI 界面，让 SSH 连接管理成为一种享受',
    qs_title: '30 秒快速上手',
    qs_desc: '克隆、构建、连接你的第一台服务器',
    step1_title: '安装 Nerd Font',
    step1_desc: '图标显示所需。例如 <code>brew install --cask font-jetbrains-mono-nerd-font</code>',
    step2_title: '克隆 &amp; 构建',
    step2_desc: '需要 Rust 1.85+。使用 <code>cargo build --release</code> 构建',
    step3_title: '运行 Wormhole',
    step3_desc: '在终端中执行 <code>wormhole</code>。设置主密码或跳过。',
    copy: '复制',
    copied: '已复制！',
    code_clone_comment: '# 克隆仓库',
    code_build_comment: '# 从源码构建',
    code_install_comment: '# 安装到 PATH',
    code_launch_comment: '# 启动！',
    kb_title: '快捷键',
    kb_key: '按键',
    kb_action: '操作',
    kb_down: '向下移动',
    kb_up: '向上移动',
    kb_panel: '切换面板',
    kb_connect: '连接到主机',
    kb_newhost: '新建主机',
    kb_newgroup: '新建分组',
    kb_edit: '编辑选中项',
    kb_delete: '删除选中项',
    kb_search: '搜索',
    kb_theme: '切换主题',
    kb_help: '帮助',
    kb_quit: '退出',
    // CTA
    cta_title: '管理 SSH，终端里就够了',
    cta_desc: '分组管理、加密凭证、即时搜索、精美主题 — 全部集成在一个 TUI 中。Rust 驱动，极速响应。一条命令安装，一个按键连接。',
    cta_install: '立即开始',
    cta_star: '在 GitHub 上 Star',
    footer_copy: '© 2025 Wormhole 贡献者。基于 MIT 许可证发布。',
    footer_license: '许可证',
  },
  ja: {
    nav_features: '機能',
    nav_screenshots: 'スクリーンショット',
    nav_quickstart: 'クイックスタート',
    hero_badge: 'ターミナルネイティブ · Rust駆動 · 超高速',
    hero_title: 'SSHマネージャー、<br>ターミナルの中に',
    hero_desc: '完全TUIベースのSSH接続マネージャー。Rustで構築。ブラウザもElectronも不要 — ターミナルだけで、滑らかで即座に応答。サーバー管理、認証情報の暗号化、ワンキーで接続。',
    hero_install: 'インストール',
    hero_source: 'ソースコード',
    features_title: '必要なものだけを、すべて',
    features_desc: 'ターミナルで活躍する開発者のための強力な機能',
    f_vault_title: '暗号化保管庫',
    f_vault_desc: 'AES-256-GCM 暗号化と Argon2id キー導出により、認証情報をローカルに安全に保管。',
    f_group_title: 'グループ管理',
    f_group_desc: '折りたたみ可能なフォルダーでホストを整理。Shift+J/K で並べ替え。',
    f_search_title: '即時検索',
    f_search_desc: 'ホスト名、アドレス、グループ全体の全文検索。ミリ秒で結果を表示。',
    f_theme_title: '4種類の内蔵テーマ',
    f_theme_desc: 'Catppuccin Mocha、Tokyo Night、Dracula、Gruvbox Dark — ライブプレビュー付き。',
    f_i18n_title: '多言語対応',
    f_i18n_desc: 'アプリ内で英語、中国語、日本語、韓国語を切り替え可能。',
    f_vim_title: 'Vim キーバインド',
    f_vim_desc: 'j/k/h/l でナビゲーション。すべてのアクションがワンキーにマップ。マウス不要。',
    screenshots_title: '実際の画面',
    screenshots_desc: '洗練された TUI で SSH 接続管理を快適に',
    qs_title: '30秒で使い始める',
    qs_desc: 'クローン、ビルド、最初のサーバーに接続',
    step1_title: 'Nerd Font のインストール',
    step1_desc: 'アイコン表示に必要。例：<code>brew install --cask font-jetbrains-mono-nerd-font</code>',
    step2_title: 'クローン &amp; ビルド',
    step2_desc: 'Rust 1.85+ が必要。<code>cargo build --release</code> でビルド',
    step3_title: 'Wormhole を起動',
    step3_desc: 'ターミナルで <code>wormhole</code> を実行。マスターパスワードを設定するかスキップ。',
    copy: 'コピー',
    copied: 'コピー済み！',
    code_clone_comment: '# リポジトリをクローン',
    code_build_comment: '# ソースからビルド',
    code_install_comment: '# PATH にインストール',
    code_launch_comment: '# 起動！',
    kb_title: 'キーバインド',
    kb_key: 'キー',
    kb_action: 'アクション',
    kb_down: '下に移動',
    kb_up: '上に移動',
    kb_panel: 'パネル切替',
    kb_connect: 'ホストに接続',
    kb_newhost: '新規ホスト',
    kb_newgroup: '新規グループ',
    kb_edit: '選択項目を編集',
    kb_delete: '選択項目を削除',
    kb_search: '検索',
    kb_theme: 'テーマ切替',
    kb_help: 'ヘルプ',
    kb_quit: '終了',
    // CTA
    cta_title: 'SSH管理は、ターミナルだけで',
    cta_desc: 'グループ化、暗号化、即時検索、美しいテーマ — すべてがひとつのTUIに。Rustで構築、超高速。コマンド1つでインストール、ワンキーで接続。',
    cta_install: '始めましょう',
    cta_star: 'GitHubでスター',
    footer_copy: '© 2025 Wormhole 貢献者。MIT ライセンスで公開。',
    footer_license: 'ライセンス',
  },
  ko: {
    nav_features: '기능',
    nav_screenshots: '스크린샷',
    nav_quickstart: '빠른 시작',
    hero_badge: '터미널 네이티브 · Rust 기반 · 초고속',
    hero_title: 'SSH 관리자,<br>터미널 안에서',
    hero_desc: '완전한 TUI 기반 SSH 연결 관리자. Rust로 구축되었습니다. 브라우저도, Electron도 필요 없습니다 — 터미널만으로 부드럽고 즉각적인 반응. 서버 관리, 자격 증명 암호화, 단일 키 입력으로 연결.',
    hero_install: '설치하기',
    hero_source: '소스 보기',
    features_title: '필요한 모든 것, 그 이상도 이하도 없이',
    features_desc: '터미널에서 작업하는 개발자를 위한 강력한 기능',
    f_vault_title: '암호화 금고',
    f_vault_desc: 'AES-256-GCM 암호화와 Argon2id 키 파생으로 자격 증명을 로컬에 안전하게 보관.',
    f_group_title: '그룹 관리',
    f_group_desc: '접을 수 있는 폴더로 호스트를 정리. Shift+J/K로 순서 변경.',
    f_search_title: '즉시 검색',
    f_search_desc: '호스트 이름, 주소, 그룹 전체의 전체 텍스트 검색. 밀리초 단위의 결과.',
    f_theme_title: '4가지 내장 테마',
    f_theme_desc: 'Catppuccin Mocha, Tokyo Night, Dracula, Gruvbox Dark — 실시간 미리보기 지원.',
    f_i18n_title: '다국어 지원',
    f_i18n_desc: '앱 내에서 영어, 중국어, 일본어, 한국어 전환 가능.',
    f_vim_title: 'Vim 단축키',
    f_vim_desc: 'j/k/h/l로 탐색. 모든 동작이 하나의 키에 매핑. 마우스 불필요.',
    screenshots_title: '실제 화면',
    screenshots_desc: '세련된 TUI로 SSH 연결 관리를 즐겁게',
    qs_title: '30초 만에 시작하기',
    qs_desc: '복제, 빌드, 첫 서버에 연결',
    step1_title: 'Nerd Font 설치',
    step1_desc: '아이콘 표시에 필요. 예: <code>brew install --cask font-jetbrains-mono-nerd-font</code>',
    step2_title: '복제 &amp; 빌드',
    step2_desc: 'Rust 1.85+ 필요. <code>cargo build --release</code>로 빌드',
    step3_title: 'Wormhole 실행',
    step3_desc: '터미널에서 <code>wormhole</code>을 실행. 마스터 비밀번호를 설정하거나 건너뛰기.',
    copy: '복사',
    copied: '복사됨!',
    code_clone_comment: '# 저장소 복제',
    code_build_comment: '# 소스에서 빌드',
    code_install_comment: '# PATH에 설치',
    code_launch_comment: '# 실행!',
    kb_title: '단축키',
    kb_key: '키',
    kb_action: '동작',
    kb_down: '아래로 이동',
    kb_up: '위로 이동',
    kb_panel: '패널 전환',
    kb_connect: '호스트에 연결',
    kb_newhost: '새 호스트',
    kb_newgroup: '새 그룹',
    kb_edit: '선택 항목 편집',
    kb_delete: '선택 항목 삭제',
    kb_search: '검색',
    kb_theme: '테마 전환',
    kb_help: '도움말',
    kb_quit: '종료',
    // CTA
    cta_title: 'SSH 관리, 터미널 안에서 끝내세요',
    cta_desc: '그룹 관리, 암호화, 즉시 검색, 아름다운 테마 — 모든 것이 하나의 TUI에. Rust로 구축된 초고속 경험. 명령 하나로 설치, 키 하나로 연결.',
    cta_install: '시작하기',
    cta_star: 'GitHub에서 스타',
    footer_copy: '© 2025 Wormhole 기여자. MIT 라이선스로 배포.',
    footer_license: '라이선스',
  },
};

const langMeta = {
  en: { flag: '\u{1F1FA}\u{1F1F8}', code: 'EN', font: 'Inter' },
  zh: { flag: '\u{1F1E8}\u{1F1F3}', code: '中文', font: "'Noto Sans SC', 'Inter'" },
  ja: { flag: '\u{1F1EF}\u{1F1F5}', code: '日本語', font: "'Noto Sans JP', 'Inter'" },
  ko: { flag: '\u{1F1F0}\u{1F1F7}', code: '한국어', font: "'Noto Sans KR', 'Inter'" },
};

let currentLang = localStorage.getItem('wormhole-lang') || 'en';

function setLang(lang) {
  currentLang = lang;
  localStorage.setItem('wormhole-lang', lang);
  const t = i18n[lang];
  const meta = langMeta[lang];

  // Update flag & code in toggle
  document.getElementById('langFlag').textContent = meta.flag;
  document.getElementById('langCode').textContent = meta.code;

  // Update html lang
  document.documentElement.lang = lang;

  // Update font family for CJK
  document.body.style.fontFamily = `${meta.font}, -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif`;

  // Update page title
  const titles = {
    en: 'Wormhole — Terminal-native SSH Manager, Built with Rust',
    zh: 'Wormhole — 终端原生 SSH 管理器，Rust 驱动',
    ja: 'Wormhole — ターミナルネイティブ SSH マネージャー、Rust駆動',
    ko: 'Wormhole — 터미널 네이티브 SSH 관리자, Rust 기반',
  };
  document.title = titles[lang];

  // Update all data-i18n elements
  document.querySelectorAll('[data-i18n]').forEach((el) => {
    const key = el.getAttribute('data-i18n');
    if (t[key]) {
      el.innerHTML = t[key];
    }
  });
}

// Language toggle dropdown
const langToggle = document.getElementById('langToggle');
const langDropdown = document.getElementById('langDropdown');

langToggle.addEventListener('click', (e) => {
  e.stopPropagation();
  langDropdown.classList.toggle('open');
});

document.addEventListener('click', () => {
  langDropdown.classList.remove('open');
});

document.querySelectorAll('.lang-option').forEach((btn) => {
  btn.addEventListener('click', () => {
    setLang(btn.dataset.lang);
    langDropdown.classList.remove('open');
  });
});

// Apply saved language on load
setLang(currentLang);

// ===== Scroll reveal animation =====
const observer = new IntersectionObserver(
  (entries) => {
    entries.forEach((entry) => {
      if (entry.isIntersecting) {
        entry.target.classList.add('visible');
      }
    });
  },
  { threshold: 0.1, rootMargin: '0px 0px -40px 0px' }
);

document.querySelectorAll('.reveal').forEach((el) => observer.observe(el));

// ===== Navbar scroll effect =====
const nav = document.querySelector('.nav');

window.addEventListener('scroll', () => {
  if (window.scrollY > 50) {
    nav.classList.add('scrolled');
  } else {
    nav.classList.remove('scrolled');
  }
}, { passive: true });

// ===== Copy to clipboard =====
document.addEventListener('click', async (e) => {
  const btn = e.target.closest('.code-block-copy');
  if (!btn) return;

  const codeBlock = btn.closest('.code-block');
  const code = codeBlock.querySelector('pre').textContent;
  const lines = code.split('\n').filter((l) => !l.trim().startsWith('#')).join('\n');

  try {
    await navigator.clipboard.writeText(lines);
  } catch {
    const ta = document.createElement('textarea');
    ta.value = lines;
    document.body.appendChild(ta);
    ta.select();
    document.execCommand('copy');
    document.body.removeChild(ta);
  }

  const t = i18n[currentLang];
  btn.textContent = t.copied;
  btn.classList.add('copied');
  setTimeout(() => {
    btn.textContent = t.copy;
    btn.classList.remove('copied');
  }, 2000);
});

// ===== Smooth scroll for anchor links =====
document.querySelectorAll('a[href^="#"]').forEach((anchor) => {
  anchor.addEventListener('click', (e) => {
    e.preventDefault();
    const target = document.querySelector(anchor.getAttribute('href'));
    if (target) {
      target.scrollIntoView({ behavior: 'smooth', block: 'start' });
    }
  });
});

// ===== Carousel =====
const track = document.getElementById('carouselTrack');
const slides = track.querySelectorAll('.carousel-slide');
const dots = document.querySelectorAll('.carousel-dot');
const prevBtn = document.getElementById('carouselPrev');
const nextBtn = document.getElementById('carouselNext');
let currentSlide = 0;
let autoPlayTimer;

function goToSlide(index) {
  currentSlide = ((index % slides.length) + slides.length) % slides.length;
  track.style.transform = `translateX(-${currentSlide * 100}%)`;
  dots.forEach((d, i) => d.classList.toggle('active', i === currentSlide));
}

function startAutoPlay() {
  stopAutoPlay();
  autoPlayTimer = setInterval(() => goToSlide(currentSlide + 1), 4000);
}

function stopAutoPlay() {
  clearInterval(autoPlayTimer);
}

prevBtn.addEventListener('click', () => { goToSlide(currentSlide - 1); startAutoPlay(); });
nextBtn.addEventListener('click', () => { goToSlide(currentSlide + 1); startAutoPlay(); });
dots.forEach((dot) => {
  dot.addEventListener('click', () => { goToSlide(+dot.dataset.index); startAutoPlay(); });
});

startAutoPlay();

// Touch swipe for carousel
let touchStartX = 0;
const viewport = document.querySelector('.carousel-viewport');
viewport.addEventListener('touchstart', (e) => { touchStartX = e.changedTouches[0].screenX; stopAutoPlay(); }, { passive: true });
viewport.addEventListener('touchend', (e) => {
  const diff = e.changedTouches[0].screenX - touchStartX;
  if (Math.abs(diff) > 50) goToSlide(diff > 0 ? currentSlide - 1 : currentSlide + 1);
  startAutoPlay();
});

// ===== Lightbox =====
const lightbox = document.getElementById('lightbox');
const lightboxImg = document.getElementById('lightboxImg');
const lightboxClose = document.getElementById('lightboxClose');
const lightboxPrev = document.getElementById('lightboxPrev');
const lightboxNext = document.getElementById('lightboxNext');
let lightboxIndex = 0;

const images = Array.from(slides).map((s) => s.querySelector('img').src);

function openLightbox(index) {
  lightboxIndex = index;
  lightboxImg.src = images[index];
  lightbox.classList.add('open');
  document.body.style.overflow = 'hidden';
}

function closeLightbox() {
  lightbox.classList.remove('open');
  document.body.style.overflow = '';
}

slides.forEach((slide, i) => {
  slide.addEventListener('click', () => openLightbox(i));
});

lightboxClose.addEventListener('click', closeLightbox);
lightbox.addEventListener('click', (e) => { if (e.target === lightbox) closeLightbox(); });
lightboxPrev.addEventListener('click', (e) => { e.stopPropagation(); lightboxIndex = (lightboxIndex - 1 + images.length) % images.length; lightboxImg.src = images[lightboxIndex]; });
lightboxNext.addEventListener('click', (e) => { e.stopPropagation(); lightboxIndex = (lightboxIndex + 1) % images.length; lightboxImg.src = images[lightboxIndex]; });

document.addEventListener('keydown', (e) => {
  if (!lightbox.classList.contains('open')) return;
  if (e.key === 'Escape') closeLightbox();
  if (e.key === 'ArrowLeft') { lightboxIndex = (lightboxIndex - 1 + images.length) % images.length; lightboxImg.src = images[lightboxIndex]; }
  if (e.key === 'ArrowRight') { lightboxIndex = (lightboxIndex + 1) % images.length; lightboxImg.src = images[lightboxIndex]; }
});
