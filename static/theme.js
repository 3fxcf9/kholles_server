const themeCycle = ["latte", "frappe", "macchiato", "mocha"].reverse();

function setTheme(themeName) {
  document.body.classList = Array.from(document.body.classList)
    .filter((c) => !c.startsWith("theme-"))
    .concat([`theme-${themeName}`]);
  localStorage.setItem("theme", themeName);
}

setTheme(localStorage.getItem("theme") || themeCycle[0]);

const toggleButton = document.createElement("div");
toggleButton.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="4"></circle><path d="M12 2v2"></path><path d="M12 20v2"></path><path d="m4.93 4.93 1.41 1.41"></path><path d="m17.66 17.66 1.41 1.41"></path><path d="M2 12h2"></path><path d="M20 12h2"></path><path d="m6.34 17.66-1.41 1.41"></path><path d="m19.07 4.93-1.41 1.41"></path></svg>`;
toggleButton.addEventListener("click", (e) => {
  setTheme(
    themeCycle[
      (themeCycle.indexOf(localStorage.getItem("theme")) + 1) %
        themeCycle.length
    ],
  );
});
toggleButton.style = `
  position: fixed;
  top: 2rem;
  right: 2rem;
  z-index: 1000;
  cursor:pointer;
  box-shadow: var(--bg-color) 0 0 15px 7px;
  background-color: var(--bg-color);
  `;
console.log(toggleButton);
document.body.appendChild(toggleButton);
