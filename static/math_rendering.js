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
    "\\F": "\\mathbb{F}",
    "\\P": "\\mathcal{P}",
    "\\K": "\\mathbb{K}",
    "\\L": "\\mathscr{L}",
    "\\B": "\\mathscr{B}",
    "\\M": "\\mathcal{M}",
    "\\E": "\\mathscr{E}",
    "\\GL": "\\mathscr{GL}",
    "\\GLM": "\\mathrm{GL}",
    "\\CM": "\\mathscr{CM}",
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
    "\\vv": "\\overrightarrow{#1}",
    "\\norm": "\\left\\lVert#1\\right\\rVert",
    "\\prop": "\\mathcal{P}",
    "\\mathquote": "\\frquote{\\text{#1}}",
    "\\arrowlim": "\\;\\xrightarrow[#1]{}\\;",
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
    "\\Img": "\\operatorname{Im}",
    "\\Id": "\\operatorname{Id}",
    "\\rg": "\\operatorname{rg}",
    "\\mat": "\\operatorname{mat}",
    "\\tr": "\\operatorname{tr}",
    "\\com": "\\operatorname{com}",
    "\\mtx": "\\begin{pmatrix}#1\\end{pmatrix}",
    "\\transp": "^{\\mkern-1.5mu\\mathsf{T}}",
    "\\tilde": "\\widetilde",
    "\\applic":
      "\\begin{array}{rcl}#1 & \\longrightarrow & #2 \\\\ #3 & \\longmapsto & #4\\end{array}",
    "\\abs": "\\left|#1\\right|",
    "\\infabs": "\\left\\lVert#1\\right\\rVert_{\\infty, #2}",
    "\\oo": "\\left]#1\\right[",
    "\\oc": "\\left]#1\\right]",
    "\\co": "\\left[#1\\right[",
    "\\cc": "\\left[#1\\right]",
    "\\iset": "\\llbracket #1 \\rrbracket",
    "\\ioo": "\\rrbracket #1 \\llbracket",
    "\\ioc": "\\rrbracket #1 \\rrbracket",
    "\\ico": "\\llbracket #1 \\llbracket",
    "\\icc": "\\rrbracket #1 \\llbracket",
    "\\where": "\\;\\middle\\vert\\;",
    "\\usim": "\\underset{#1}{\\sim}",
    // "\\transp": "^{\\top}",
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

  document.querySelectorAll("span[data-math-style]").forEach((element) => {
    const math = element.textContent;
    const style = element.getAttribute("data-math-style"); // "inline" or "display"

    const renderElement = document.createElement(
      style === "display" ? "div" : "span",
    );

    if (element.classList.length) {
      renderElement.classList.add(...element.classList);
    }

    element.replaceWith(renderElement);

    try {
      katex.render(math, renderElement, {
        displayMode: style === "display",
        throwOnError: false,
        macros: macros,
      });
    } catch (err) {
      console.error("KaTeX rendering error:", err);
    }
  });
});
