window.addEventListener("load", async () => {
  await window.TreeSitter.init();

  async function setLanguage(language) {
    const grammar = await window.TreeSitter.Language.load(
      `tree-sitter-languages/${language}/grammar.wasm`,
    );
    const response = await fetch(
      `tree-sitter-languages/${language}/highlights.scm`,
    );
    const highlightQueries = await response.text();
    await window.TS.parser.setLanguage(grammar);
    window.TS.activeGrammar = grammar;
    window.TS.query = grammar.query(highlightQueries);
  }

  // Highlighting code adapted from https://adrian.schoenig.me/blog/2022/05/27/tree-sitter-highlighting-in-jekyll/
  function highlight(el) {
    const code = el.innerText;
    const tree = window.TS.parser.parse(code);
    var adjusted = "";
    var lastEnd = 0;

    TS.query.matches(tree.rootNode).forEach((match) => {
      const className = match.captures[0].name.replaceAll(".", "-");
      const text = match.captures[0].node.text;
      const start = match.captures[0].node.startIndex;
      const end = match.captures[0].node.endIndex;

      if (start < lastEnd) {
        return; // avoid duplicate matches for the same text
      }
      if (start > lastEnd) {
        adjusted += code.substring(lastEnd, start);
      }

      adjusted += `<span class="${className}">${text}</span>`;
      lastEnd = end;
    });

    if (lastEnd < code.length) {
      adjusted += code.substring(lastEnd);
    }
    el.innerHTML = adjusted;
  }

  window.TS = {
    parser: new window.TreeSitter(),
    activeGrammar: null,
    query: null,
    setLanguage,
    highlight,
  };

  await TS.setLanguage("rust");
});
