use axum::{
    Router,
    extract::Path,
    http::{StatusCode, header},
    response::{Html, IntoResponse},
    routing::get,
};

const CSS: &str = include_str!("../assets/site.css");
const AVATAR: &[u8] = include_bytes!("../assets/avatar.png");

const GITHUB_URL: &str = "https://github.com/DeluxeRaph";
const LINKEDIN_URL: &str = "https://www.linkedin.com/in/raphael-nembhard-701b41180/";
const X_URL: &str = "https://x.com/villagefarmerr?s=21&t=UwOpwNOVe8JHdycxnEfQnQ";
const PROJECT_SCRIPT: &str = r##"
<script>
const projects = {
  "longeth": {
    name: "longETH",
    language: "Solidity",
    type: "DeFi",
    description: "LongETH is a Uniswap v4 rehypothecation hook that compounds yield into ETH.",
    url: "https://github.com/DeluxeRaph/longETH",
    tags: ["DeFi", "Uniswap v4", "Hook"]
  },
  "seawatch": {
    name: "SeaWatch",
    language: "Python",
    type: "Hardware",
    description: "Mesh-networked smart buoys that detect and classify vessels using computer vision and relay sightings over LoRa.",
    url: "https://github.com/DeluxeRaph/SeaWatch",
    tags: ["Hardware", "Computer Vision", "LoRa"]
  },
  "run-money": {
    name: "run-money",
    language: "Other",
    type: "App",
    description: "Run Money is a savings game you play with your running buddies.",
    url: "https://github.com/DeluxeRaph/run-money",
    tags: ["App", "Savings", "Game"]
  },
  "flaunchgg-contracts": {
    name: "flaunchgg-contracts",
    language: "Solidity",
    type: "Contracts",
    description: "The Flaunch protocol is a platform designed to change how memecoins are launched and traded.",
    url: "https://github.com/flayerlabs/flaunchgg-contracts",
    tags: ["Contracts", "Flaunch", "Uniswap v4"]
  },
  "flaunch-sdk": {
    name: "flaunch-sdk",
    language: "TypeScript",
    type: "SDK",
    description: "A TypeScript SDK to make interacting with the Flaunch protocol and Uniswap V4 easier.",
    url: "https://github.com/flayerlabs/flaunch-sdk",
    tags: ["SDK", "Flaunch", "Uniswap v4"]
  },
  "hydrex-sdk": {
    name: "hydrex-sdk",
    language: "TypeScript",
    type: "SDK",
    description: "TypeScript SDK for the Hydrex concentrated-liquidity AMM on Base.",
    url: "https://github.com/hydrexfi/hydrex-sdk",
    tags: ["SDK", "Hydrex", "Base"]
  },
  "dgl": {
    name: "DGL",
    language: "Other",
    type: "Project",
    description: "Curated project entry. Add the exact GitHub URL when ready.",
    url: "https://github.com/DeluxeRaph",
    tags: ["Project", "Needs Link"]
  },
  "ai-dex": {
    name: "ai_dex",
    language: "Python",
    type: "AI",
    description: "An experimental DEX that swaps using an intent-based aggregator and an AI chatbot interface.",
    url: "https://github.com/DeluxeRaph/ai_dex",
    tags: ["AI", "DEX", "Intents"]
  },
  "swoupon-hook": {
    name: "swoupon-hook",
    language: "Solidity",
    type: "DeFi",
    description: "A Uniswap v4 hook swapping rewards mechanism where swappers accumulate Swoupon tokens that can be redeemed for fee discounts.",
    url: "https://github.com/DeluxeRaph/swoupon-hook",
    tags: ["DeFi", "Uniswap v4", "Rewards"]
  },
  "super-dca-contracts": {
    name: "super-dca-contracts",
    language: "Solidity",
    type: "Open Source",
    description: "Open-source contract work for Super DCA Pools.",
    url: "https://github.com/Super-DCA-Tech/super-dca-contracts",
    tags: ["Open Source", "Contracts", "DCA"]
  },
  "base-rust": {
    name: "Base",
    language: "Rust",
    type: "Open Source",
    description: "Open-source Rust contributions to Base, including Flashblocks and transaction test work.",
    url: "https://github.com/base/base/pulls?q=author%3ADeluxeRaph",
    tags: ["Open Source", "Rust", "Base"]
  },
  "ike-contracts": {
    name: "ike-contracts",
    language: "Solidity",
    type: "Open Source",
    description: "Added lock functionality for Governance NFT transfers in ike-contracts.",
    url: "https://github.com/WaterCoolerStudiosInc/ike-contracts/pull/28",
    tags: ["Open Source", "Contracts", "Governance"]
  }
};

function renderProject(projectKey) {
  const project = projects[projectKey];
  const detail = document.querySelector("#project-detail");
  if (!project || !detail) return;

  detail.innerHTML = `
    <span class="tiny-label">${project.language}</span>
    <h3>${project.name}</h3>
    <p>${project.description}</p>
    <div class="badge-row">
      ${project.tags.map((tag) => `<span class="badge">${tag}</span>`).join("")}
    </div>
    <div class="button-row compact-row">
      <a class="button-95" href="${project.url}">Open GitHub</a>
    </div>
  `;
}

function filterProjects() {
  const search = document.querySelector("#project-search")?.value.toLowerCase() || "";
  const language = document.querySelector("#project-language")?.value || "all";
  const type = document.querySelector("#project-type")?.value || "all";
  const rows = Array.from(document.querySelectorAll(".project-row"));
  let firstVisible = null;

  rows.forEach((row) => {
    const project = projects[row.dataset.project];
    const haystack = [project.name, project.language, project.description, ...project.tags].join(" ").toLowerCase();
    const matchesSearch = haystack.includes(search);
    const matchesLanguage = language === "all" || row.dataset.language === language;
    const matchesType = type === "all" || row.dataset.type === type;
    const visible = matchesSearch && matchesLanguage && matchesType;
    row.hidden = !visible;
    row.classList.remove("is-active");
    if (visible && !firstVisible) firstVisible = row;
  });

  const detail = document.querySelector("#project-detail");
  if (!firstVisible) {
    if (detail) {
      detail.innerHTML = `<span class="tiny-label">No match</span><h3>No projects found</h3><p>Try a different search, language, or type filter.</p>`;
    }
    return;
  }

  firstVisible.classList.add("is-active");
  renderProject(firstVisible.dataset.project);
}

document.addEventListener("DOMContentLoaded", () => {
  document.querySelectorAll(".project-row").forEach((row) => {
    row.addEventListener("click", () => {
      document.querySelectorAll(".project-row").forEach((item) => item.classList.remove("is-active"));
      row.classList.add("is-active");
      renderProject(row.dataset.project);
    });
  });

  document.querySelector("#project-search")?.addEventListener("input", filterProjects);
  document.querySelector("#project-language")?.addEventListener("change", filterProjects);
  document.querySelector("#project-type")?.addEventListener("change", filterProjects);

  const taskList = document.querySelector("#task-window-list");
  const windows = Array.from(document.querySelectorAll(".window[data-window-title]"));

  function setActiveWindow(windowElement) {
    windows.forEach((item) => item.classList.remove("is-focused"));
    document.querySelectorAll(".task-window").forEach((item) => item.classList.remove("is-active"));

    if (!windowElement) return;
    windowElement.classList.add("is-focused");
    document.querySelector(`.task-window[data-window-target="${windowElement.id}"]`)?.classList.add("is-active");
  }

  function restoreWindow(windowElement, shouldScroll = true) {
    if (!windowElement) return;
    windowElement.classList.remove("is-minimized");
    setActiveWindow(windowElement);
    if (shouldScroll) {
      windowElement.scrollIntoView({ behavior: "smooth", block: "start" });
    }
  }

  if (taskList) {
    windows.forEach((windowElement, index) => {
      if (!windowElement.id) {
        windowElement.id = `window-${index + 1}`;
      }

      const taskButton = document.createElement("button");
      taskButton.type = "button";
      taskButton.className = "task-pill task-window";
      taskButton.dataset.windowTarget = windowElement.id;
      taskButton.textContent = windowElement.dataset.windowTitle || windowElement.id;
      taskButton.addEventListener("click", () => {
        restoreWindow(windowElement);
      });
      taskList.appendChild(taskButton);

      windowElement.addEventListener("pointerdown", () => setActiveWindow(windowElement));
    });

    setActiveWindow(windows[0]);
  }

  document.querySelectorAll("[data-window-action='minimize']").forEach((button) => {
    button.addEventListener("click", (event) => {
      const windowElement = event.currentTarget.closest(".window");
      if (!windowElement) return;
      windowElement.classList.add("is-minimized");
      document.querySelector(`.task-window[data-window-target="${windowElement.id}"]`)?.classList.remove("is-active");
    });
  });

  document.querySelectorAll("a[href^='#']").forEach((link) => {
    link.addEventListener("click", () => {
      const target = document.querySelector(link.getAttribute("href"));
      if (target?.classList.contains("window")) {
        restoreWindow(target, false);
      }
    });
  });
});
</script>
"##;

#[derive(Clone, Copy)]
struct BlogPost {
    slug: &'static str,
    title: &'static str,
    date: &'static str,
    summary: &'static str,
    body: &'static str,
}

const POSTS: &[BlogPost] = &[];

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home))
        .route("/blog", get(blog_index))
        .route("/blog/{slug}", get(blog_post))
        .route("/assets/site.css", get(styles))
        .route("/assets/avatar.png", get(avatar));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("bind local server");

    println!("Listening on http://127.0.0.1:3000");
    axum::serve(listener, app).await.expect("serve app");
}

async fn home() -> Html<String> {
    Html(page(
        "Raphael Nembhard",
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
            <a href="https://github.com/DeluxeRaph">GitHub</a>
            <a href="https://x.com/villagefarmerr?s=21&t=UwOpwNOVe8JHdycxnEfQnQ">X</a>
            <a href="https://www.linkedin.com/in/raphael-nembhard-701b41180/">LinkedIn</a>
          </nav>
          <div class="window-body">
            <div class="hero-grid">
              <div class="sunken hero-copy">
                <span class="kicker">Software Builder · villagefarmer on X</span>
                <h1>Raphael Nembhard</h1>
                <p class="lede">I build software, test new tech, learn fast, and keep pushing the limits of what I can do.</p>
                <div class="button-row">
                  <a class="button-95" href="#projects">Open Projects</a>
                  <a class="button-95" href="https://github.com/DeluxeRaph">GitHub</a>
                  <a class="button-95" href="https://x.com/villagefarmerr?s=21&t=UwOpwNOVe8JHdycxnEfQnQ">X</a>
                  <a class="button-95" href="/blog">Blog</a>
                </div>
              </div>
              <aside class="sunken info-panel" aria-label="Profile details">
                <div>
                  <span class="tiny-label">Current file</span>
                  <h2>deluxe_agent.exe</h2>
                  <p>A coming-soon Cloudflare agent that uses the deluxe-skills repo as its main skill library, with x402-style paid access for people who want to run it.</p>
                  <div class="button-row compact-row">
                    <a class="button-95" href="https://github.com/DeluxeRaph/deluxe-skills">View Skills Repo</a>
                    <span class="button-95 is-disabled">Payment Gate Pending</span>
                  </div>
                </div>
                <div>
                  <span class="tiny-label">Access flow</span>
                  <div class="meter" aria-label="Agent launch readiness"><span></span></div>
                </div>
                <div class="badge-row">
                  <span class="badge">Cloudflare</span>
                  <span class="badge">x402</span>
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
        "##.to_owned()
            + &blog_cards(POSTS)
            + r##"
            </div>
          </div>
          <div class="status-bar">
            <div class="status-cell"><a href="/blog">Open all posts</a></div>
            <div class="status-cell">0 files</div>
          </div>
        </section>
        "##,
    ))
}

async fn blog_index() -> Html<String> {
    Html(page(
        "Blog - Raphael Nembhard",
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
            <a href="https://github.com/DeluxeRaph">GitHub</a>
            <a href="https://x.com/villagefarmerr?s=21&t=UwOpwNOVe8JHdycxnEfQnQ">X</a>
            <a href="https://www.linkedin.com/in/raphael-nembhard-701b41180/">LinkedIn</a>
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
        .to_owned()
            + &blog_cards(POSTS)
            + r#"
          </div>
        </section>
        "#,
    ))
}

async fn blog_post(Path(slug): Path<String>) -> impl IntoResponse {
    let Some(post) = POSTS.iter().find(|post| post.slug == slug) else {
        return (
            StatusCode::NOT_FOUND,
            Html(page(
                "Post not found",
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
                "#,
            )),
        );
    };

    (
        StatusCode::OK,
        Html(page(
            post.title,
            &format!(
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
            ),
        )),
    )
}

async fn styles() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "text/css; charset=utf-8")], CSS)
}

async fn avatar() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "image/png")], AVATAR)
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

fn page(title: &str, content: impl AsRef<str>) -> String {
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
    {PROJECT_SCRIPT}
  </body>
</html>"#
    )
}
