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
            <a href="/shop">Shop</a>
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
              <aside class="sunken info-panel" aria-label="Profile details">
                <div>
                  <span class="tiny-label">Current file</span>
                  <h2>deluxe_agent.exe</h2>
                  <p>Live beta: a paid-agent storefront for deluxe-skills. The shop exposes an x402-style paid mint route now, while the Cloudflare-hosted agent runtime is still being built in public.</p>
                  <div class="button-row compact-row">
                    <a class="button-95" href="/shop">Open Live Beta</a>
                    <a class="button-95" href="https://github.com/DeluxeRaph/deluxe-skills">View Skills Repo</a>
                  </div>
                </div>
                <div>
                  <span class="tiny-label">Access flow</span>
                  <div class="meter" aria-label="Agent launch readiness"><span></span></div>
                </div>
                <div class="badge-row">
                  <span class="badge">Live beta</span>
                  <span class="badge">x402-style access</span>
                  <span class="badge">Paid Agent</span>
                  <span class="badge">deluxe-skills</span>
                </div>
              </aside>
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

pub(crate) fn shop() -> &'static str {
    r#"
        <section class="window shop-window" id="agent-shop" data-window-title="AGENT_SHOP.EXE">
          <div class="title-bar">
            <span>AGENT_SHOP.EXE</span>
            <div class="window-actions" aria-hidden="true">
              <button class="window-button" type="button" data-window-action="minimize">_</button>
              <button class="window-button is-disabled" type="button" disabled aria-label="Maximize shop">□</button>
              <button class="window-button is-disabled" type="button" disabled aria-label="Close shop">×</button>
            </div>
          </div>
          <nav class="menu-bar" aria-label="Primary">
            <a href="/">Home</a>
            <a href="/shop">Shop</a>
            <a href="/blog">Blog</a>
            <a href="https://github.com/DeluxeRaph/deluxe-skills">Skills Repo</a>
            <a href="https://github.com/DeluxeRaph/personal_website">Source</a>
          </nav>
          <div class="window-body shop-hero-body">
            <div class="shop-grid">
              <article class="sunken hero-copy shop-copy">
                <span class="kicker">Live beta · x402-style paid access</span>
                <h1>0xFarmer Agent Shop</h1>
                <p class="lede">A public beta storefront for paid agent experiments. Today it demonstrates an x402-style $1 USDC route on Base for minting an 8-bit 0xFarmer keepsake; next it will gate access to a Cloudflare-hosted agent powered by the <strong>deluxe-skills</strong> library.</p>
                <p class="beta-note"><strong>Active development:</strong> this is real infrastructure, but it is still a live beta. Expect rough edges, small changes, and visible build progress while the paid agent flow is finalized.</p>
                <div class="badge-row">
                  <span class="badge">$1 USDC</span>
                  <span class="badge">Base mainnet</span>
                  <span class="badge">x402 v2 challenge</span>
                  <span class="badge">Live beta</span>
                </div>
              </article>
              <aside class="sunken shop-counter">
                <div class="pixel-shop" aria-hidden="true">
                  <div class="lantern left"></div>
                  <div class="lantern right"></div>
                  <div class="awning"></div>
                  <div class="sign">0xFARMER 屋</div>
                  <div class="door"></div>
                  <div class="window-glow"></div>
                </div>
                <form id="mint-form" class="mint-form">
                  <label>Wallet address
                    <input class="field-95" id="mint-wallet" name="wallet" placeholder="0x..." autocomplete="off" required>
                  </label>
                  <label>Image prompt
                    <input class="field-95" id="mint-prompt" name="prompt" value="farmer cat tending a tiny lantern shop" maxlength="80">
                  </label>
                  <button class="button-95 shop-primary" type="submit">Start $1 x402 mint</button>
                  <p class="form-help">Submitting from the browser requests the payment challenge and shows the AgentCash command to complete it.</p>
                </form>
              </aside>
            </div>
          </div>
          <div class="window-body shop-details-body">
            <div class="shop-panel-grid">
              <article class="card x402-card">
                <span class="tiny-label">How x402 works here</span>
                <h3>Pay only when your client retries with proof</h3>
                <ol class="step-list">
                  <li>The mint API returns <code>402 Payment Required</code> with Base USDC payment requirements.</li>
                  <li>An x402-capable client such as AgentCash pays and retries with the payment header.</li>
                  <li>The server settles the payment, then generates metadata and records the mint.</li>
                </ol>
                <div class="command-card">
                  <span>AgentCash beta command</span>
                  <code>npx agentcash@latest fetch https://0xfarmer.com/api/shop/mint -m POST -b '{"wallet":"0x...","prompt":"farmer cat"}'</code>
                </div>
              </article>
              <article class="card minted-card">
                <span class="tiny-label">Latest mint</span>
                <div id="mint-result" class="mint-result">No local mints yet. Start a paid mint to see the result here.</div>
              </article>
            </div>
          </div>
          <div class="status-bar">
            <div class="status-cell" id="shop-status">Loading shop status…</div>
            <div class="status-cell">live beta</div>
          </div>
        </section>
    "#
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
            <a href="/shop">Shop</a>
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
            <a class="shortcut" href="/shop">
              <span class="shortcut-icon">店</span>
              <span>Shop</span>
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
