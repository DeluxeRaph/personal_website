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

  const shopStatus = document.querySelector("#shop-status");
  const mintForm = document.querySelector("#mint-form");
  const mintResult = document.querySelector("#mint-result");

  async function refreshShopStatus() {
    if (!shopStatus) return;
    try {
      const response = await fetch("/api/shop/status");
      const status = await response.json();
      shopStatus.textContent = `Minted ${status.minted}/${status.max_supply} · $${status.price_usd} · ${status.cooldown_seconds / 60} min cooldown · ${status.network}`;
    } catch (error) {
      shopStatus.textContent = `Shop status unavailable: ${error.message}`;
    }
  }

  function renderMint(mint) {
    if (!mintResult) return;
    mintResult.replaceChildren();

    const title = document.createElement("strong");
    title.textContent = `Token #${mint.token_id}`;
    const prompt = document.createElement("p");
    prompt.textContent = mint.prompt;
    const tokenUri = document.createElement("p");
    tokenUri.textContent = `Metadata: ${mint.token_uri}`;
    mintResult.append(title, prompt, tokenUri);
  }

  mintForm?.addEventListener("submit", async (event) => {
    event.preventDefault();
    const form = new FormData(mintForm);
    const body = {
      wallet: form.get("wallet"),
      prompt: form.get("prompt")
    };

    if (mintResult) {
      mintResult.innerHTML = `<strong>Requesting x402 challenge…</strong><p>This browser demo checks the paid route first, then gives you the AgentCash command for settlement.</p>`;
    }

    try {
      const challenge = await fetch("/api/shop/mint", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(body)
      });
      const challengeJson = await challenge.json();

      if (challenge.status !== 402) {
        mintResult.textContent = challengeJson.error || "Expected a 402 payment request.";
        return;
      }

      const commandBody = JSON.stringify(body).replace(/'/g, "'\''");
      mintResult.innerHTML = `
        <strong>x402 challenge ready</strong>
        <p>The route returned <code>402 Payment Required</code>. Complete the beta flow with an x402-capable client:</p>
        <pre class="inline-command">npx agentcash@latest fetch ${window.location.origin}/api/shop/mint -m POST -b '${commandBody}'</pre>
        <p class="form-help">The API settles payment before minting. Browser wallets need an x402 v2 client that retries with the payment header.</p>
      `;
    } catch (error) {
      mintResult.textContent = `Could not request x402 challenge: ${error.message}`;
    }
  });

  refreshShopStatus();
});
