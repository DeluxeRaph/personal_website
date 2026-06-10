use crate::{
    config::{GITHUB_URL, LINKEDIN_URL, X_URL},
    content::{BlogPost, POSTS},
};

pub(crate) fn home() -> String {
    format!(
        r##"
        <section class="window" id="home" data-window-title="RAPHAEL.EXE">
          <div class="title-bar">
            <span>RAPHAEL.EXE</span>
            <div class="window-actions" aria-hidden="true">
              <button class="window-button" type="button" data-window-action="minimize">_</button>
              <button class="window-button is-disabled" type="button" disabled aria-label="Maximize RAPHAEL.EXE">□</button>
              <button class="window-button is-disabled" type="button" disabled aria-label="Close RAPHAEL.EXE">×</button>
            </div>
          </div>
          <nav class="menu-bar" aria-label="Primary">
            <a href="#about">About</a>
            <a href="#projects">Projects</a>
            <a href="#work">Work</a>
            <a href="/blog">Blog</a>
            <a href="{GITHUB_URL}">GitHub</a>
            <a href="{X_URL}">X</a>
            <a href="{LINKEDIN_URL}">LinkedIn</a>
          </nav>
          <div class="window-body">
            <div class="hero-grid">
              <div class="sunken hero-copy">
                <span class="kicker">Software Builder · villagefarmer on X</span>
                <h1>Raphael Nembhard</h1>
                <p class="lede">I build software, test new tech, learn fast, and keep pushing the limits of what I can do.</p>
                <div class="button-row">
                  <a class="button-95" href="#projects">Open Projects</a>
                  <a class="button-95" href="{GITHUB_URL}">GitHub</a>
                  <a class="button-95" href="{X_URL}">X</a>
                  <a class="button-95" href="/blog">Blog</a>
                </div>
              </div>
            </div>
          </div>
          <div class="status-bar">
            <div class="status-cell">Ready: personal site running on Rust + Axum</div>
            <div class="status-cell">1996-ish</div>
          </div>
        </section>

        <section class="window" id="about" data-window-title="ABOUT_ME.DOC">
          <div class="title-bar">
            <span>ABOUT_ME.DOC</span>
            <div class="window-actions">
              <button class="window-button" type="button" data-window-action="minimize">_</button>
              <button class="window-button is-disabled" type="button" disabled aria-label="About help">?</button>
            </div>
          </div>
          <div class="window-body">
            <article class="sunken">
              <p class="about-paragraph">Hello, my name is Raphael Nembhard, also known as villagefarmer on X. I am a software builder who loves testing new tech, learning fast, and pushing the limits of what I can do. Outside of software, I have a bigger dream: owning my own farm one day after retirement. Family is a huge part of what drives me. I work hard to make them proud and believe we can make the world better if we move with integrity, respect people, and keep showing up with purpose.</p>
            </article>
          </div>
        </section>

        <section class="window" id="projects" data-window-title="PROJECT_INDEX.EXE">
          <div class="title-bar">
            <span>PROJECT_INDEX.EXE</span>
            <div class="window-actions">
              <button class="window-button" type="button" data-window-action="minimize">_</button>
              <button class="window-button is-disabled" type="button" disabled aria-label="Resize PROJECT_INDEX.EXE">↕</button>
            </div>
          </div>
          <div class="menu-bar project-toolbar" aria-label="Project filters">
            <label>
              Search
              <input id="project-search" class="field-95" type="search" placeholder="repo, tag, language">
            </label>
            <label>
              Language
              <select id="project-language" class="field-95">
                <option value="all">All</option>
                <option value="solidity">Solidity</option>
                <option value="python">Python</option>
                <option value="rust">Rust</option>
                <option value="typescript">TypeScript</option>
                <option value="other">Other</option>
              </select>
            </label>
            <label>
              Type
              <select id="project-type" class="field-95">
                <option value="all">All</option>
                <option value="ai">AI</option>
                <option value="app">App</option>
                <option value="contracts">Contracts</option>
                <option value="defi">DeFi</option>
                <option value="hardware">Hardware</option>
                <option value="open source">Open Source</option>
                <option value="project">Project</option>
                <option value="sdk">SDK</option>
              </select>
            </label>
          </div>
          <div class="window-body">
            <div class="project-shell">
              <div class="project-list" id="project-list" aria-label="Projects">
                <button class="project-row is-active" type="button" data-project="longeth" data-language="solidity" data-type="defi">
                  <span>longETH</span>
                  <span>Solidity</span>
                  <span>DeFi</span>
                </button>
                <button class="project-row" type="button" data-project="seawatch" data-language="python" data-type="hardware">
                  <span>SeaWatch</span>
                  <span>Python</span>
                  <span>Hardware</span>
                </button>
                <button class="project-row" type="button" data-project="run-money" data-language="other" data-type="app">
                  <span>run-money</span>
                  <span>Other</span>
                  <span>App</span>
                </button>
                <button class="project-row" type="button" data-project="flaunchgg-contracts" data-language="solidity" data-type="contracts">
                  <span>flaunchgg-contracts</span>
                  <span>Solidity</span>
                  <span>Contracts</span>
                </button>
                <button class="project-row" type="button" data-project="flaunch-sdk" data-language="typescript" data-type="sdk">
                  <span>flaunch-sdk</span>
                  <span>TypeScript</span>
                  <span>SDK</span>
                </button>
                <button class="project-row" type="button" data-project="hydrex-sdk" data-language="typescript" data-type="sdk">
                  <span>hydrex-sdk</span>
                  <span>TypeScript</span>
                  <span>SDK</span>
                </button>
                <button class="project-row" type="button" data-project="dgl" data-language="other" data-type="project">
                  <span>DGL</span>
                  <span>Other</span>
                  <span>Project</span>
                </button>
                <button class="project-row" type="button" data-project="ai-dex" data-language="python" data-type="ai">
                  <span>ai_dex</span>
                  <span>Python</span>
                  <span>AI</span>
                </button>
                <button class="project-row" type="button" data-project="swoupon-hook" data-language="solidity" data-type="defi">
                  <span>swoupon-hook</span>
                  <span>Solidity</span>
                  <span>DeFi</span>
                </button>
                <button class="project-row" type="button" data-project="super-dca-contracts" data-language="solidity" data-type="open source">
                  <span>super-dca-contracts</span>
                  <span>Solidity</span>
                  <span>Open Source</span>
                </button>
                <button class="project-row" type="button" data-project="base-rust" data-language="rust" data-type="open source">
                  <span>Base</span>
                  <span>Rust</span>
                  <span>Open Source</span>
                </button>
                <button class="project-row" type="button" data-project="ike-contracts" data-language="solidity" data-type="open source">
                  <span>ike-contracts</span>
                  <span>Solidity</span>
                  <span>Open Source</span>
                </button>
              </div>
              <article class="project-detail sunken" id="project-detail" aria-live="polite">
                <span class="tiny-label">Solidity</span>
                <h3>longETH</h3>
                <p>LongETH is a Uniswap v4 rehypothecation hook that compounds yield into ETH.</p>
                <div class="badge-row">
                  <span class="badge">DeFi</span>
                  <span class="badge">Uniswap v4</span>
                  <span class="badge">Hook</span>
                </div>
                <div class="button-row compact-row">
                  <a class="button-95" href="https://github.com/DeluxeRaph/longETH">Open GitHub</a>
                </div>
              </article>
            </div>
          </div>
          <div class="status-bar">
            <div class="status-cell">Github Projects</div>
            <div class="status-cell">12 files</div>
          </div>
        </section>

        <section class="window" id="work" data-window-title="WORK_HISTORY.DOC">
          <div class="title-bar">
            <span>WORK_HISTORY.DOC</span>
            <div class="window-actions">
              <button class="window-button" type="button" data-window-action="minimize">_</button>
              <button class="window-button is-disabled" type="button" disabled aria-label="Resize WORK_HISTORY.DOC">↕</button>
            </div>
          </div>
          <div class="window-body">
            <div class="section-grid">
              <article class="card">
                <span class="tiny-label">Current</span>
                <h3>Flayer Labs</h3>
                <p>Public profile lists Flayer Labs as Raphael's current company. Add the exact title, dates, and strongest shipped work here.</p>
              </article>
              <article class="card">
                <span class="tiny-label">Builder</span>
                <h3>Independent Projects</h3>
                <p>Protocol experiments, agent tooling, hardware ideas, and SDK work that show a bias toward building and testing new technology.</p>
              </article>
              <article class="card">
                <span class="tiny-label">Education</span>
                <h3>Atrium Academy</h3>
                <p>Completed in 2024, according to the public LinkedIn profile. This can become a cleaner education/training entry once details are final.</p>
              </article>
            </div>
          </div>
        </section>

        <section class="window" id="blog-preview" data-window-title="BLOG_FOLDER">
          <div class="title-bar">
            <span>BLOG_FOLDER</span>
            <div class="window-actions">
              <button class="window-button" type="button" data-window-action="minimize">_</button>
              <button class="window-button is-disabled" type="button" disabled aria-label="Favorite BLOG_FOLDER">★</button>
            </div>
          </div>
          <div class="window-body">
            <div class="blog-list">
    "##
    ) + &blog_cards(POSTS)
        + r##"
            </div>
          </div>
          <div class="status-bar">
            <div class="status-cell"><a href="/blog">Open all posts</a></div>
            <div class="status-cell">0 files</div>
          </div>
        </section>
    "##
}

pub(crate) fn blog_index() -> String {
    format!(
        r#"
        <section class="window" id="blog-window" data-window-title="BLOG_FOLDER">
          <div class="title-bar">
            <span>BLOG_FOLDER</span>
            <div class="window-actions" aria-hidden="true">
              <button class="window-button" type="button" data-window-action="minimize">_</button>
              <button class="window-button is-disabled" type="button" disabled aria-label="Maximize BLOG_FOLDER">□</button>
              <button class="window-button is-disabled" type="button" disabled aria-label="Close BLOG_FOLDER">×</button>
            </div>
          </div>
          <nav class="menu-bar" aria-label="Primary">
            <a href="/">Home</a>
            <a href="/blog">Blog</a>
            <a href="{GITHUB_URL}">GitHub</a>
            <a href="{X_URL}">X</a>
            <a href="{LINKEDIN_URL}">LinkedIn</a>
          </nav>
          <div class="window-body">
            <div class="sunken">
              <span class="kicker">Writing</span>
              <h1>Blog</h1>
              <p class="lede">No posts yet. This folder is ready for essays, dev notes, and experiments when they exist.</p>
            </div>
          </div>
          <div class="window-body blog-list">
    "#
    ) + &blog_cards(POSTS)
        + r#"
          </div>
        </section>
    "#
}

pub(crate) fn blog_not_found() -> &'static str {
    r#"
        <section class="window">
          <div class="title-bar"><span>404.TXT</span><span class="window-button">×</span></div>
          <div class="window-body">
            <div class="sunken">
              <h1>Post not found</h1>
              <p>That blog file is not on this desktop yet.</p>
              <a class="button-95" href="/blog">Back to Blog</a>
            </div>
          </div>
        </section>
    "#
}

pub(crate) fn blog_post(post: &BlogPost) -> String {
    format!(
        r#"
        <section class="window">
          <div class="title-bar">
            <span>{}.TXT</span>
            <span class="window-button">×</span>
          </div>
          <nav class="menu-bar" aria-label="Primary">
            <a href="/">Home</a>
            <a href="/blog">Blog</a>
            <a href="https://github.com/DeluxeRaph">GitHub</a>
          </nav>
          <div class="window-body">
            <article class="sunken">
              <span class="kicker">{}</span>
              <h1>{}</h1>
              <p class="lede">{}</p>
              <p>{}</p>
              <a class="button-95" href="/blog">Back to Blog</a>
            </article>
          </div>
        </section>
        "#,
        post.slug.to_uppercase(),
        post.date,
        post.title,
        post.summary,
        post.body
    )
}

fn blog_cards(posts: &[BlogPost]) -> String {
    if posts.is_empty() {
        return r#"
        <article class="card empty-state">
          <span class="tiny-label">empty folder</span>
          <h3>No blog posts yet</h3>
          <p>This section is intentionally blank until Raphael has posts to publish.</p>
        </article>
        "#
        .to_owned();
    }

    posts
        .iter()
        .map(|post| {
            format!(
                r#"
                <article class="card blog-post">
                  <div class="date-tile">{}</div>
                  <div>
                    <span class="tiny-label">blog file</span>
                    <h3><a href="/blog/{}">{}</a></h3>
                    <p>{}</p>
                  </div>
                </article>
                "#,
                post.date, post.slug, post.title, post.summary
            )
        })
        .collect()
}

pub(crate) fn layout(title: &str, content: impl AsRef<str>) -> String {
    let content = content.as_ref();

    format!(
        r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <meta name="description" content="Raphael Nembhard's Rust and Axum personal site.">
    <title>{title}</title>
    <link rel="stylesheet" href="/assets/site.css">
    <script src="/assets/site.js" defer></script>
  </head>
  <body>
    <main class="desktop">
      <div class="top-shell">
        <aside class="avatar-dock" aria-label="Desktop shortcuts">
          <div class="avatar-ring">
            <img src="/assets/avatar.png" alt="Illustrated profile image for Raphael Nembhard">
          </div>
          <div class="shortcut-list">
            <a class="shortcut" href="/">
              <span class="shortcut-icon">⌂</span>
              <span>Home</span>
            </a>
            <a class="shortcut" href="/blog">
              <span class="shortcut-icon">✎</span>
              <span>Blog</span>
            </a>
            <a class="shortcut" href="{GITHUB_URL}">
              <span class="shortcut-icon">GH</span>
              <span>GitHub</span>
            </a>
            <a class="shortcut" href="{X_URL}">
              <span class="shortcut-icon">X</span>
              <span>villagefarmer</span>
            </a>
            <a class="shortcut" href="{LINKEDIN_URL}">
              <span class="shortcut-icon">in</span>
              <span>LinkedIn</span>
            </a>
          </div>
        </aside>
        <div>{content}</div>
      </div>
    </main>
    <footer class="taskbar">
      <a class="start-button" href="/">Start</a>
      <div class="task-window-list" id="task-window-list" aria-label="Open windows"></div>
      <div class="task-pill task-status">Axum online</div>
    </footer>
  </body>
</html>"#
    )
}
