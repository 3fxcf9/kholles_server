document.addEventListener("DOMContentLoaded", function () {
  const macros = {
    "\\leq": "\\leqslant",
    "\\geq": "\\geqslant",
    "\\Sol": "\\mathcal{S}",
    "\\N": "\\mathbb{N}",
    "\\Z": "\\mathbb{Z}",
    "\\Q": "\\mathbb{Q}",
    "\\R": "\\mathbb{R}",
    "\\C": "\\mathbb{C}",
    "\\U": "\\mathbb{U}",
    "\\P": "\\mathbb{P}",
    "\\K": "\\mathbb{K}",
    "\\L": "\\mathscr{L}",
    "\\GL": "\\mathscr{GL}",
    "\\Func": "\\mathcal{F}",
    "\\Cont": "\\mathcal{C}",
    "\\Diff": "\\mathcal{D}",
    "\\conj": "\\overline",
    "\\Re": "\\mathscr{R\\!e}",
    "\\Im": "\\mathscr{I\\!\\!m}",
    "\\acos": "\\operatorname{Arccos}",
    "\\asin": "\\operatorname{Arcsin}",
    "\\atan": "\\operatorname{Arctan}",
    "\\ch": "\\operatorname{ch}",
    "\\sh": "\\operatorname{sh}",
    "\\th": "\\operatorname{th}",
    "\\set": "\\{\\,#1\\,\\}",
    "\\cgm": "\\equiv #1 \\left[#2\\right]",
    "\\ncgm": "\\not\\equiv #1 \\left[#2\\right]",
    "\\lient": "[\\![",
    "\\rient": "]\\!]",
    "\\iset": "\\lient #1 \\rient",
    "\\vv": "\\overrightarrow{#1}",
    "\\norm": "\\left\\lVert#1\\right\\rVert",
    "\\prop": "\\mathcal{P}",
    "\\mathquote": "\\frquote{\\text{#1}}",
    "\\arrowlim": "\\ \\xrightarrow[\\;#1 \\to #2\\;]{}\\ ",
    "\\textlim": "\\lim\\limits_{#1}",
    "\\eps": "\\varepsilon",
    "\\ph": "\\varphi",
    "\\lbda": "\\lambda",
    "\\dd": "\\mathrm{d}",
    "\\prob": "\\mathbb{P}",
    "\\expect": "\\mathbb{E}",
    "\\variance": "\\mathbb{V}",
    "\\Vect": "\\operatorname{Vect}",
    "\\Ker": "\\operatorname{Ker}",
    "\\Imf": "\\operatorname{Im}",
    "\\Id": "\\operatorname{Id}",
  };

  renderMathInElement(document.body, {
    delimiters: [
      { left: "$$", right: "$$", display: true },
      { left: "$", right: "$", display: false },
      // { left: "\\(", right: "\\)", display: false },
      // { left: "\\[", right: "\\]", display: true },
    ],
    throwOnError: false,
  });

  document
    .querySelectorAll("code.math-inline, code.math-display")
    .forEach((element) => {
      let math = element.textContent;
      // Create a new element for rendering
      const renderElement = document.createElement(
        element.classList.contains("math-display") ? "div" : "span",
      );
      // Replace the code element with the new element
      element.parentNode.replaceChild(renderElement, element);
      try {
        katex.render(math, renderElement, {
          displayMode: element.classList.contains("math-display"),
          throwOnError: false,
          macros: macros,
        });
      } catch (e) {
        console.error("KaTeX rendering error:", e);
      }
    });
});
