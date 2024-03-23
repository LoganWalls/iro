window.addEventListener("load", async () => {
  async function setLanguage(name) {
    // Initialize tree-sitter if needed
    if (!window.TS.parser) {
      await window.TreeSitter.init();
      window.TS.parser = new window.TreeSitter();
    }

    // Load the language parser & queries
    const language = await window.TreeSitter.Language.load(
      `${document.baseURI}treesitter/${name}/grammar.wasm`,
    );
    const response = await fetch(
      `${document.baseURI}treesitter/${name}/highlights.scm`,
    );
    const highlightQueries = await response.text();
    await window.TS.parser.setLanguage(language);
    window.TS.activeLanguage = language;
    window.TS.query = language.query(highlightQueries);
  }

  // Highlighting code adapted from https://adrian.schoenig.me/blog/2022/05/27/tree-sitter-highlighting-in-jekyll/
  function highlight(code) {
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
    return adjusted;
  }

  window.TS = {
    parser: null,
    activeLanguage: null,
    query: null,
    setLanguage,
    highlight,
  };
});
